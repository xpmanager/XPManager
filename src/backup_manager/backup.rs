
use std::path::PathBuf;
use super::ArgMatches;
use crate::{
    loglib,
    filelib,
    errorlib,
    dblib
};

/// Backup the password manager database.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::FileNotEncrypted`
/// - `errorlib::ExitErrorCode::PMDatabaseNotFound`
/// 
/// ### Example:
/// ```
/// // find it at: `./to/passwords.db.x`
/// password_manager("./to".to_string());
/// ```
fn password_manager(path: String) {
    let logger = loglib::Logger::new("password-manager-backup");
    let pm_db_state = filelib::pm::db_state();
    if pm_db_state == filelib::FileState::Decrypted {
        logger.error(
            "password manager found decrypted!, please encrypt it and try to backup!", 
            errorlib::ExitErrorCode::FileNotEncrypted
        );
    } else if pm_db_state == filelib::FileState::NotFound {
        logger.error(
            "password manager found empty!, please save some password and try to backup!", 
            errorlib::ExitErrorCode::PMDatabaseNotFound
        );
    }
    let encryption_db_path = filelib::pm::get_encrypted_db_path();
    let backup_name = encryption_db_path
        .file_name()
        .unwrap();
    filelib::copy(
        encryption_db_path
            .to_str()
            .unwrap()
            .to_owned(), 
        PathBuf::new()
            .join(path)
            .join(&backup_name)
            .to_str()
            .unwrap()
            .to_owned()
    );
    dblib::log::register("created password manager backup.", filelib::log::get_log_db_path());
    logger.info("created the password manager backup successfully.");
}

/// Backup the log manager database.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::LMDatabaseNotFound`
/// 
/// ### Example:
/// ```
/// // find it at: `./to/xpm-log.db`
/// logs_manager("./to".to_string());
/// ```
fn logs_manager(path: String) {
    let logger = loglib::Logger::new("logs-manager-backup");
    let log_db_path = filelib::log::get_log_db_path()
            .to_str()
            .unwrap()
            .to_owned();
    let log_db_state = filelib::get_file_state(
        log_db_path.clone()
    );
    if log_db_state == filelib::FileState::NotFound {
        logger.error(
            "logs manager found empty!", 
            errorlib::ExitErrorCode::LMDatabaseNotFound
        );
    }
    filelib::copy(
        log_db_path.clone(), 
        PathBuf::new()
            .join(path)
            .join(
                PathBuf::new()
                    .join(log_db_path)
                    .file_name()
                    .unwrap()
            ).to_str()
            .unwrap()
            .to_owned()
    );
    dblib::log::register("created logs manager backup.", filelib::log::get_log_db_path());
    logger.info("created the logs manager backup successfully.");
}

pub fn main(command: &ArgMatches) {
    let logger = loglib::Logger::new("backup");
    let path = command.get_one::<String>("PATH").unwrap();
    let is_password = *command.get_one("password").unwrap_or(&false);
    let is_log = *command.get_one("log").unwrap_or(&false);
    if  is_password {
        password_manager(path.clone());
    }
    if is_log {
        logs_manager(path.clone());
    }
    if !is_password && !is_log {
        logger.error(
            "Run with 'backup-manager backup --help'",
            errorlib::ExitErrorCode::MissingArg
        )
    }
}