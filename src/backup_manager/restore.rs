
use std::{
    collections::HashMap, 
    path::PathBuf
};
use super::ArgMatches;
use crate::{
    dblib, 
    encryption_manager::decrypt_file, 
    errorlib, 
    filelib, 
    loglib, 
    password_manager::PMDatabaseEncrption, 
    utilities
};

/// Save the passwords `HashMap` to the password manager database.
/// 
/// ### Example:
/// ```
/// let passwords_map = vec_to_hash_map(
///     vec![
///         dblib::pm::PasswordInfoForm {
///             id: 32,
///             name: "XPManager".to_string(),
///             password: "pass123".to_string(),
///             create_at: "2025-05-24 15:59:12".to_string(),
///             update_at: "2025-05-24 15:59:12".to_string()
///         }
///     ]
/// );
/// hash_map_to_pm_db(passwords_map, PathBuf::new().join("./dir/passwords.db"));
/// ```
fn hash_map_to_pm_db(data: HashMap<String, String>, pm_db_path: PathBuf) {
    let logger = loglib::Logger::new("password-restore");
    for (name, password) in data {
        // merge the new password to the database.
        dblib::pm::save_password(
            pm_db_path.clone(),
            name.clone(), 
            password
        );
        dblib::log::register(
            &format!("'{}' saved successfully.", name), 
            filelib::log::get_log_db_path()
        );
        logger.info(
            &format!("'{}' restored successfully.", name)
        );
    }
}

/// Parse `dblib::pm::PasswordInfoForm` list to `HashMap`.
/// 
/// ### Example:
/// ```
/// let map = vec_to_hash_map(
///     vec![
///         dblib::pm::PasswordInfoForm {
///             id: 32,
///             name: "XPManager".to_string(),
///             password: "pass123".to_string(),
///             create_at: "2025-05-24 15:59:12".to_string(),
///             update_at: "2025-05-24 15:59:12".to_string()
///         }
///     ]
/// );
/// for (name, password) in map {
///     println!("{}: {}", name, password);
/// }
/// ```
fn vec_to_hash_map(data: Vec<dblib::pm::PasswordInfoForm>) -> HashMap<String, String> {
    let mut passwords: HashMap<String, String> = HashMap::new();
    for pass in data {
        passwords.insert(
            pass.name.clone(), 
            pass.password.clone()
        );
    }
    return passwords;
}

/// Restore log manager database.
/// 
/// ### Example:
/// ```
/// logs_manager("./dir/logs.db".to_string());
/// ```
fn logs_manager(path: String) {
    let logger = loglib::Logger::new("logs-restore");
    let logs_manager_db_path = filelib::log::get_log_db_path()
        .to_str()
        .unwrap()
        .to_owned();
    let logs_db_state = filelib::get_file_state(logs_manager_db_path.clone());
    if logs_db_state == filelib::FileState::Decrypted {
        filelib::wipe_delete(logs_manager_db_path.clone());
    }
    filelib::copy(path, logs_manager_db_path);
    logger.info("restore the logs manager database successfully.");
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("restore");
    let path = command.get_one::<String>("PATH").unwrap();
    let file_state = filelib::get_file_state(path.clone());
    let pm_db_state = filelib::pm::db_state();
    
    let is_password = *command.get_one("password").unwrap_or(&false);
    let is_xpmv1 = *command.get_one::<bool>("xpmv1").unwrap_or(&false);
    let is_password_json = *command.get_one::<bool>("password-json").unwrap_or(&false);
    let is_logs = *command.get_one("log").unwrap_or(&false);

    // can not restore --password-json or --log when the file is encrypted,
    // file must be decrypted befor the restore, user can use enctryption-manager
    // to decrypt the file and then restore it. This is because we have 2 different
    // encryption techniques: XPManager v2.0, XPManager v1.0.
    if file_state == filelib::FileState::Encrypted && (is_logs || is_password_json) {
        logger.error(
            "can not restore --password-json or --log when the file is encrypted!", 
            errorlib::ExitErrorCode::Input
        )
    }  else if file_state == filelib::FileState::NotFound {
        logger.error(
            "file not found!", 
            errorlib::ExitErrorCode::FileNotFound
        )
    }

    // Ensure secure encryption and decryption in password manager database.
    if is_password || is_xpmv1 || is_password_json {
        let mut _hash_map_data: HashMap<String, String> = HashMap::new();
        let pm_db_path = filelib::pm::get_decrypted_db_path();
        let mut pm_db_encryption = PMDatabaseEncrption::new();
        let mut _is_pm_db_decrypted: bool = false;

        if is_password_json {
            _hash_map_data = filelib::read_json(path.clone());
        } else if is_xpmv1 {
            if file_state == filelib::FileState::Decrypted {
                logger.error(
                    "if your password manager v1.0 database is not encrypted use --password-json !!", 
                    errorlib::ExitErrorCode::Input
                )
            }
            decrypt_file::xpmv1_decryption(
                path.clone(), 
                utilities::input("Enter file decrpytion key: ")
            );
            let decrpted_path = filelib::make_decrypt_path(path.clone());
            _hash_map_data = filelib::read_json(decrpted_path.clone());
            filelib::wipe_delete(decrpted_path);
        } else if is_password {
            let mut _decrpted_path: PathBuf = PathBuf::new().join(path.clone());
            if file_state == filelib::FileState::Encrypted {
                decrypt_file::decrypt(
                    path.clone(),
                    utilities::input("Enter file decrpytion key: ")
                );
                _decrpted_path = PathBuf::new().join(filelib::make_decrypt_path(path.clone()));
            }
            _hash_map_data = vec_to_hash_map (
                dblib::pm::get_passwords(_decrpted_path.clone())
            );
            if file_state == filelib::FileState::Encrypted { 
                filelib::wipe_delete(_decrpted_path.to_str().unwrap().to_owned());
            }
        }

        // Ensure that the password manager database created if not exist,
        // And decrypted if it is encrypted.
        if pm_db_state == filelib::FileState::NotFound {
            filelib::create_file(
                pm_db_path.clone()
            );
            dblib::pm::create_passwords_table(pm_db_path.clone());
            dblib::log::register("create passwords table", filelib::log::get_log_db_path());
        } else if pm_db_state == filelib::FileState::Encrypted {
            logger.warning("database is encrypted!");
            pm_db_encryption.decrypt();
            logger.start();
            _is_pm_db_decrypted = true;
            logger.info("password manager database decrypted successfully.");
        }

        // Save the restore passwords to password manager database. We just
        // merge the new passwords in the password manager database.
        hash_map_to_pm_db(_hash_map_data, pm_db_path.clone());

        // Encrypt the password manager database if it is decrypted.
        if _is_pm_db_decrypted {
            pm_db_encryption.encrypt();
            logger.info("password manager database encrypted successfully.");
        }
    } else if is_logs {
        logs_manager(path.clone());
    } else {
        logger.error(
            "Run with 'backup-manager restore --help'",
            errorlib::ExitErrorCode::MissingArg
        )
    }
}