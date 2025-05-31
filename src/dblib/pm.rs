use crate::{
    loglib,
    errorlib,
    filelib
};
use super::{
    params,
    Connection,
    PathBuf,
    Tabled
};

/// The password DTO.
#[derive(Tabled)]
pub struct PasswordInfoForm {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub create_at: String,
    pub update_at: String,
}

/// Create the password table in the database.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBCreateTable`
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// dblib::pm::create_passwords_table(pm_db_path);
/// ```
pub fn create_passwords_table(password_manager_db_path: PathBuf) {
    let logger = loglib::Logger::new("create-passwords-table");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Err(_) = conn.execute("
            CREATE TABLE IF NOT EXISTS passwords(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
                create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                update_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ", []) {
            // We run this function when the passwords.db first created,
            // So if the execute of creating the password table not ok
            // we must delete the db file to tell xpm 'there is no database'.
            // Disconnect the conn from the file befor you delete it.
            conn.close().unwrap();
            filelib::delete_file(password_manager_db_path);
            logger.error(
                "can NOT create the password table!", 
                errorlib::ExitErrorCode::DBCreateTable
            );
        }
        logger.info("passwords table created successfully.");
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Save password.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBInsert`
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// dblib::pm::save_password(pm_db_path, "XPManager".to_string(), "pass123".to_string());
/// ```
pub fn save_password(password_manager_db_path: PathBuf, name: String, password: String) {
    let logger = loglib::Logger::new("save-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Err(_) = conn.execute("
                INSERT INTO passwords 
                (name, password, update_at) VALUES 
                (?1, ?2, CURRENT_TIMESTAMP)
            ", 
            params![name.clone(), password]
        ) {
            logger.error(
                "can NOT save the password!", 
                errorlib::ExitErrorCode::DBInsert
            );
        }
        logger.info(&format!("'{}' saved successfully.", name) );
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Find one password using string.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Exmaple: 
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let passwords: Vec<dblib::pm::PasswordInfoForm> = dblib::pm::find_password(pm_db_path, "XPManager".to_string());
/// for pass in passwords {
///     println!("{}: {}", pass.name, pass.password)
/// }
/// ```
pub fn find_password(password_manager_db_path: PathBuf, string: String) -> Vec<PasswordInfoForm> {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Ok(mut stmt) = conn.prepare("SELECT * FROM passwords WHERE name LIKE ?1") {
            let pattern = format!("%{}%", string);
            let password: Result<Vec<PasswordInfoForm>, _> = stmt.query_map(params![pattern], |row| {
                Ok(
                    PasswordInfoForm {
                        id: row.get::<_, i32>(0).unwrap(),
                        name: row.get::<_, String>(1).unwrap(),
                        password: row.get::<_, String>(2).unwrap(), 
                        create_at: row.get::<_, String>(3).unwrap(),
                        update_at: row.get::<_, String>(4).unwrap()
                    }
                )
            })
                .unwrap()
                .collect();
            return password.unwrap();
        }
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            password_manager_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

/// Get all passwords.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let passwords: Vec<dblib::pm::PasswordInfoForm> = dblib::pm::get_passwords(pm_db_path);
/// for pass in passwords {
///     println!("{}: {}", pass.name, pass.password)
/// }
/// ```
pub fn get_passwords(password_manager_db_path: PathBuf) -> Vec<PasswordInfoForm> {
    find_password(password_manager_db_path, "".to_owned())
}

/// Update the password.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let rows_affected = dblib::pm::update_password(pm_db_path, "1".to_string(), "pass123".to_string());
/// if rows_affected > 0 {
///     println!("{} passwords updated.", rows_affected);
/// } else {
///     println!("No password found!");
/// }
/// ```
pub fn update_password(password_manager_db_path: PathBuf, id: String, password: String) -> usize {
    let logger = loglib::Logger::new("update-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows = conn.execute("
                    UPDATE passwords 
                    SET password = ?1,
                    update_at = CURRENT_TIMESTAMP 
                    WHERE id=?2
                ",
                params![password, id]
            ).unwrap_or(0);
            return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Update the password name.
/// 
/// ### Exit: 
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let rows_affected = dblib::pm::update_password_name(pm_db_path, "1".to_string(), "XPManager".to_string());
/// if rows_affected > 0 {
///     println!("{} passwords updated.", rows_affected);
/// } else {
///     println!("No password found!");
/// }
/// ```
pub fn update_password_name(password_manager_db_path: PathBuf, id: String, name: String) -> usize {
    let logger = loglib::Logger::new("update-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows= conn.execute("
                    UPDATE passwords 
                    SET name = ?1,
                    update_at = CURRENT_TIMESTAMP 
                    WHERE id=?2
                ",
                params![name, id]
            ).unwrap_or(0);
            return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Get the number of passwords.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example: 
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let number_of_passwords = dblib::pm::get_passwords_number(pm_db_path);
/// println!("number of password: {}", number_of_passwords);
/// ```
pub fn get_passwords_number(password_manager_db_path: PathBuf) -> usize {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        return conn.query_row(
            "SELECT COUNT(*) FROM passwords", 
            [],
            |row| row.get::<_, usize>(0),
        ).unwrap();
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            password_manager_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

/// Delete password by id.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let pm_db_path = PathBuf::new().join("./dir/passwords.db");
/// let rows_affected = dblib::pm::delete_password(pm_db_path, "1".to_string());
/// if rows_affected > 0 {
///     println!("{} passwords deleted.", rows_affected);
/// } else {
///     println!("No password found!");
/// }
/// ```
pub fn delete_password(password_manager_db_path: PathBuf, id: String) -> usize {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows = conn.execute("
                DELETE FROM passwords 
                WHERE id=?1
            ",
            params![id]
        ).unwrap_or(0);
        return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}


#[cfg(test)]
mod tests {
    
    use std::path::PathBuf;
    use super::filelib::create_file;

    #[test]
    fn create_passwords_table() {
        let temp_dir = PathBuf::new()
            .join("./temp/create_passwords_table");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path);

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn save_password() {
        let temp_dir = PathBuf::new()
            .join("./temp/save_password");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test".to_string(), 
            "test123".to_string()
        );

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn find_password() {
        let temp_dir = PathBuf::new()
            .join("./temp/find_password");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test".to_string(), 
            "test123".to_string()
        );
        let passwords = super::find_password(
            db_path.clone(), 
            "test".to_string()
        );
        assert!(
            passwords[0].name     == "test" &&
            passwords[0].password == "test123",
            "Password NOT match!!",
        );

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn get_passwords() {
        let temp_dir = PathBuf::new()
            .join("./temp/get_passwords");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test-1".to_string(), 
            "test123".to_string()
        );
        super::save_password(
            db_path.clone(),
            "test-2".to_string(), 
            "test123".to_string()
        );
        let passwords = super::get_passwords(db_path.clone() );
        assert_eq!(passwords.len(), 2, "Can NOT get all passwords!!");
        assert!(
            passwords[0].name     == "test-1" &&
            passwords[0].password == "test123",
            "Password NOT match!!",
        );
        assert!(
            passwords[1].name     == "test-2" &&
            passwords[1].password == "test123",
            "Password NOT match!!",
        );

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn update_password_and_name() {
        let temp_dir = PathBuf::new()
            .join("./temp/update_password_and_name");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test".to_string(), 
            "test123".to_string()
        );
        let mut passwords = super::get_passwords(db_path.clone() );
        assert!(
            passwords[0].name     == "test" &&
            passwords[0].password == "test123",
            "Password NOT match!!",
        );
        super::update_password(
            db_path.clone(), 
            1.to_string(), 
            "new123".to_string()
        );
        passwords = super::get_passwords(db_path.clone() );
        assert_eq!(passwords[0].password,"new123", "The password NOT updated!!");
        super::update_password_name(
            db_path.clone(), 
            1.to_string(),
            "new".to_string()
        );
        passwords = super::get_passwords(db_path.clone() );
        assert_eq!( passwords[0].name, "new", "The password name NOT updated!!" );

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn get_passwords_number() {
        let temp_dir = PathBuf::new()
            .join("./temp/get_passwords_number");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test".to_string(), 
            "test123".to_string()
        );
        let number = super::get_passwords_number(db_path.clone() );
        assert_eq!(number, 1, "Get password number NOT match!!");

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn delete_password() {
        let temp_dir = PathBuf::new()
            .join("./temp/delete_password");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_passwords_table(db_path.clone());
        super::save_password(
            db_path.clone(),
            "test".to_string(), 
            "test123".to_string()
        );
        let mut number = super::get_passwords_number(db_path.clone() );
        assert_eq!(number, 1, "Number of passwords NOT match!!");
        super::delete_password(db_path.clone(), 1.to_string());
        number = super::get_passwords_number(db_path.clone() );
        assert_eq!(number, 0, "Can NOT delete the password!!");

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp dir!!");
    }
}