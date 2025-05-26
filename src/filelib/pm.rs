use super::{
    XPM_EXTENSION,
    data_dir,
    PathBuf,
    loglib,
    errorlib,
    FileState
};

/// Get the encrypted password manager database full path.
/// It will return the database path in the user's 
/// data directory: 
/// - Linux: `/home/{user}/.local/share/XPManager/data/passwords.db.x`
/// - MacOS: `/Users/{user}/Library/Application Support/XPManager/data/passwords.db.x`
/// - Windows: `C:\Users\{user}\AppData\Roaming\XPManager\data\passwords.db.x`
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::SystemDataDirNotFound`
/// 
/// ### Example:
/// ```
/// let pm_db_path = filelib::pm::get_encrypted_db_path();
/// println!("Path is: {}", pm_db_path);
/// ```
pub fn get_encrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-encrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join(
            format!("XPManager/data/passwords.db.{}", XPM_EXTENSION)
        );
    } else {
        logger.error(
            "can NOT get the system data directory path!",
            errorlib::ExitErrorCode::SystemDataDirNotFound
        );
    }
}

/// Get the password manager database full path.
/// It will return the database path in the user's 
/// data directory: 
/// - Linux: `/home/{user}/.local/share/XPManager/data/passwords.db`
/// - MacOS: `/Users/{user}/Library/Application Support/XPManager/data/passwords.db`
/// - Windows: `C:\Users\{user}\AppData\Roaming\XPManager\data\passwords.db`
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::SystemDataDirNotFound`
/// 
/// ### Example:
/// ```
/// let pm_db_path = filelib::pm::get_decrypted_db_path();
/// println!("Path is: {}", pm_db_path);
/// ```
pub fn get_decrypted_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-pm-decrypted-db-path");
    if let Some(data_path) = data_dir() {
        return data_path.join("XPManager/data/passwords.db");
    } else {
        logger.error(
            "can NOT get the system data directory path!", 
            errorlib::ExitErrorCode::SystemDataDirNotFound
        );
    }
}

/// Get the password manager database state. It will
/// return `FileState` enum: 
/// - Encrypted
/// - Decrypted
/// - NotFound
/// 
/// ### Example:
/// ```
/// let pm_db_state = filelib::pm::warning_encrypt_database();
/// if pm_db_state == filelib::FileState::Encrypted {
///     println!("password manager database is encrypted.");
/// } else if pm_db_state == filelib::FileState::Decrypted {
///     println!("password manager database is decrypted.");
/// } else {
///     println!("password manager database not found!");
/// }
/// ```
pub fn db_state() -> FileState {
    if get_encrypted_db_path().exists() {
        return FileState::Encrypted;
    } else if get_decrypted_db_path().exists() {
        return FileState::Decrypted;
    }
    return FileState::NotFound;
}

/// Check the password manager database
/// if it is not encrypted.
/// 
/// ### Example:
/// ```
/// // it will print warning if the db is decrypted.
/// filelib::pm::warning_encrypt_database();
/// ```
pub fn warning_encrypt_database() {
    let logger = loglib::Logger::new("check-password-manager-database");
    if db_state() == FileState::Decrypted {
        logger.warning("password manager database found NOT encrypted!!");
        logger.warning("please use 'password-manager encrypt' to encrypt it!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_encrypted_db_path() {
        let db_path = super::get_encrypted_db_path();
        assert_eq!(
            db_path,
            super::data_dir()
                .expect("can NOT get the system data directory path!!")
                .join("XPManager/data/passwords.db.x"),
            "Encryption path NOT match!!"
        );
    }

    #[test]
    fn get_decrypted_db_path() {
        let db_path = super::get_decrypted_db_path();
        assert_eq!(
            db_path,
            super::data_dir()
                .expect("can NOT get the system data directory path!!")
                .join("XPManager/data/passwords.db"),
            "Encryption path NOT match!!"
        );
    }

    #[test]
    fn db_state() {
        let en_db_path = super::get_encrypted_db_path();
        let de_db_path = super::get_decrypted_db_path();
        let state = if en_db_path.exists() {
            super::FileState::Encrypted
        } else if de_db_path.exists() {
            super::FileState::Decrypted
        } else {
            super::FileState::NotFound
        };
        let result = state == super::db_state();
        assert_eq!( result, true, "Password manager database state NOT match!!" );
    }
}