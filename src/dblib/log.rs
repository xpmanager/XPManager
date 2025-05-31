use super::{
    Connection,
    PathBuf,
    Tabled
};
use crate::{
    loglib,
    filelib,
    errorlib
};

/// The log DTO.
#[derive(Tabled)]
pub struct LogInfoForamt {
    pub id: u32,
    pub log: String,
    pub create_at: String
}

/// Create the log table in the database.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBCreateTable`
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// dblib::log::create_log_table(log_db_path);
/// ```
fn create_log_table(log_db_path: PathBuf) {
    let logger = loglib::Logger::new("create-logs-table");
    if let Ok(conn) = Connection::open(&log_db_path) {
        if let Err(_) = conn.execute("
            CREATE TABLE IF NOT EXISTS logs(
                id INTEGER PRIMARY KEY,
                log TEXT NOT NULL,
                create_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ", []) {
            conn.close().unwrap();
            filelib::delete_file(log_db_path);
            logger.error(
                "can NOT create the logs table!", 
                errorlib::ExitErrorCode::DBCreateTable
            );
        }
        logger.info("logs table created successfully.");
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Register log.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::LMDatabaseEncrypted`
/// - `errorlib::ExitErrorCode::DBInsert`
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// dblib::log::register("'XPManager' seved.".to_string(), log_db_path);
/// ```
pub fn register(log: &str, log_db_path: PathBuf) {
    let logger = loglib::Logger::new("register-log");
    let log_db_path_str = log_db_path.to_str().unwrap().to_string();
    let mut log_db_state = filelib::get_file_state(log_db_path_str.clone());

    // cerate the db file and the logs table
    if log_db_state == filelib::FileState::NotFound {
        filelib::create_file(log_db_path.clone());
        create_log_table(log_db_path.clone());
    } 

    // check db is not encrypted
    log_db_state = filelib::get_file_state(log_db_path_str);
    if log_db_state == filelib::FileState::Encrypted {
        logger.error(
            "Your log manager database is encrypted, Please decrypt it and try again!!",
            errorlib::ExitErrorCode::LMDatabaseEncrypted
        );
    }

    // register the log
    if let Ok(conn) = Connection::open(&log_db_path) {
        if let Err(_) = conn.execute("INSERT INTO logs (log) VALUES (?1)", [log]) {
            conn.close().unwrap();
            filelib::delete_file(log_db_path);
            logger.error(
                "can NOT insert into the logs table!", 
                errorlib::ExitErrorCode::DBInsert
            );
        }
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Delete all logs.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// let rows_affected = dblib::log::delete_all(log_db_path);
/// if rows_affected > 0 {
///     println!("{} logs deleted.", rows_affected);
/// } else {
///     println!("No log found!");
/// }
/// ```
pub fn delete_all(log_db_path: PathBuf) -> usize {
    let logger = loglib::Logger::new("delete-all-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        return conn
            .execute("DELETE FROM logs", [])
            .unwrap_or(0);
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

/// Get logs by a string. or get all logs.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// let logs: Vec<dblib::log::LogInfoForamt> = dblib::log::get_logs(
///     log_db_path, 
///     0, // the number of logs to return, 0 means all logs
///     "".to_string() // the search string, "" means all logs
/// );
/// for log in logs {
///     println!(
///         "{}: {} - {}", 
///         log.id, 
///         log.log, 
///         log.create_at
///     );
/// }
/// ```
pub fn get_logs(log_db_path: PathBuf, length: u16, string: String) -> Vec<LogInfoForamt> {
    let logger = loglib::Logger::new("get-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        let sql = if length > 0 {
            &format!("
                    SELECT id, log, create_at 
                    FROM logs
                    ORDER BY create_at DESC, id DESC
                    LIMIT {}
                ", 
                length
            )
        } else if string.len() > 0 {
            &format!("
                    SELECT id, log, create_at 
                    FROM logs
                    WHERE log LIKE '%{}%'
                ", 
                string
            )
        } else {
            "SELECT id, log, create_at FROM logs"
        };
        let e = conn.prepare(sql);
        if let Ok(mut stmt) = e {
            let password: Result<Vec<LogInfoForamt>, _> = stmt.query_map([], |row| {
                Ok(
                    LogInfoForamt {
                        id: row.get::<_, u32>(0).unwrap(),
                        log: row.get::<_, String>(1).unwrap(),
                        create_at: row.get::<_, String>(2).unwrap(),
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
            log_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

/// Get log by date. 
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example: 
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// let logs: Vec<dblib::log::LogInfoForamt> = dblib::log::get_logs_by_date(
///     log_db_path, 
///     (2023, 12, 9) // format: (year, month, day)
/// );
/// for log in logs {
///     println!(
///         "{}: {} - {}", 
///         log.id, 
///         log.log, 
///         log.create_at
///     );
/// }
/// ```
pub fn get_logs_by_date(log_db_path: PathBuf, date: (u16, u8, u8)) -> Vec<LogInfoForamt> {
    let logger = loglib::Logger::new("get-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        let mut sql: String = "
            SELECT id, log, create_at
            FROM logs
            WHERE log LIKE '%%'
        ".to_owned();
        // SELECT * FROM logs
        // WHER log LIKE '%%' -- to use the AND date
        //      AND strftime('%Y', create_at) = year
        //      AND strftime('%m', create_at) = month
        //      AND strftime('%d', create_at) = day
        if date.0 > 0 {
            sql += &format!(" AND strftime('%Y', create_at) = '{:04}'", date.0);
        }
        if date.1 > 0 {
            sql += &format!(" AND strftime('%m', create_at) = '{:02}'", date.1);
        }
        if date.2 > 0 {
            sql += &format!(" AND strftime('%d', create_at) = '{:02}'", date.2);
        }
        let e = conn.prepare(&sql);
        if let Ok(mut stmt) = e {
            let password = stmt.query_map(
                [],
                |row| {
                    Ok(
                        LogInfoForamt {
                            id: row.get::<_, u32>(0).unwrap(),
                            log: row.get::<_, String>(1).unwrap(),
                            create_at: row.get::<_, String>(2).unwrap(),
                        }
                    )
            })
                .unwrap()
                .collect::<Result<Vec<_>, _>>();
            return password.unwrap();
        }
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            log_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

/// Delete log using id.
/// 
/// ### Exit:
/// - `errorlib::ExitErrorCode::DBConnection`
/// 
/// ### Example:
/// ```
/// let log_db_path = PathBuf::new().join("./dir/logs.db");
/// let rows_affected = dblib::log::delete_one(log_db_path, "1".to_string());
/// if rows_affected > 0 {
///     println!("{} logs deleted.", rows_affected);
/// } else {
///     println!("No log found!");
/// }
/// ```
pub fn delete_one(log_db_path: PathBuf, id: String) -> usize {
    let logger = loglib::Logger::new("delete-log");
    if let Ok(conn) = Connection::open(&log_db_path) {
        return conn.execute("
                DELETE FROM logs 
                WHERE id=?1
            ",
            [id]
        ).unwrap_or(0);
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}


#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use super::filelib::create_file;
    use chrono::Local;

    #[test]
    fn create_log_table() {
        let temp_dir = PathBuf::new()
            .join("./temp/create_log_table");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        create_file(db_path.clone());
        assert_eq!(db_path.exists(), true, "Can NOT create the test file!!");

        // This will panic and exit the program if an error occurs.
        super::create_log_table(db_path.clone());

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn log_register() {
        let temp_dir = PathBuf::new()
            .join("./temp/log_register");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");

        // This will panic and exit the program if an error occurs.
        super::register("test", db_path.clone());
        let logs = super::get_logs(
            db_path.clone(),
            0, // if 0 then all logs
            "".to_string() // if empty then all logs
        );
        assert_eq!(logs.len(), 1, "Number of logs NOT match!!");
        assert_eq!(logs[0].log, "test", "Log NOT match!!");

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn delete_all_logs() {
        let temp_dir = PathBuf::new()
            .join("./temp/delete_all_logs");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");

        // This will panic and exit the program if an error occurs.
        super::register("test-1", db_path.clone());
        super::register("test-2", db_path.clone());
        let mut logs = super::get_logs(
            db_path.clone(),
            0, // if 0 then all logs
            "".to_string() // if empty then all logs
        );
        assert_eq!(logs.len(), 2, "Number of logs NOT match!!");
        super::delete_all(db_path.clone());
        logs = super::get_logs(
            db_path.clone(),
            0,
            "".to_string()
        );
        assert_eq!(logs.len(), 0, "Can NOT delete all logs!!");

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn get_logs() {
        let temp_dir = PathBuf::new()
            .join("./temp/get_logs");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");

        // This will panic and exit the program if an error occurs.
        super::register("test-1", db_path.clone());
        super::register("test-2", db_path.clone());
        super::register("test-3", db_path.clone());
        let mut logs = super::get_logs(
            db_path.clone(),
            2, // if 2 then return 2 logs
            "".to_string() // if empty then all logs
        );
        assert_eq!(logs.len(), 2, "Number of logs NOT match!!");
        logs = super::get_logs(
            db_path.clone(),
            0, // get all logs
            "1".to_string() // search any log have "1" in it.
        );
        assert_eq!(logs.len(), 1, "Search for log, length must be 1!!");
        assert_eq!(logs[0].log, "test-1", "Search for log, log must be test-1!!");

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn get_logs_by_date() {
        let temp_dir = PathBuf::new()
            .join("./temp/get_logs_by_date");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");
        let local = Local::now();
        let year = local
            .format("%Y")
            .to_string()
            .parse::<u16>()
            .expect("Can NOT parse to u16!!");
        let month = local
            .format("%m")
            .to_string()
            .parse::<u8>()
            .expect("Can NOT parse to u8!!");
        let day = local
            .format("%d")
            .to_string()
            .parse::<u8>()
            .expect("Can NOT parse to u8!!");

        // This will panic and exit the program if an error occurs.
        super::register("test", db_path.clone());
        let mut logs = super::get_logs_by_date(
            db_path.clone(), 
            (year, 0, 0) // (year, month, day)
        );
        assert_eq!(logs.len(), 1, "Number of logs with year NOT match!!");
        logs = super::get_logs_by_date(
            db_path.clone(), 
            (year, month, 0) // (year, month, day)
        );
        assert_eq!(logs.len(), 1, "Number of logs with month NOT match!!");
        logs = super::get_logs_by_date(
            db_path.clone(), 
            (year, month, day) // (year, month, day)
        );
        assert_eq!(logs.len(), 1, "Number of logs with day NOT match!!");

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }

    #[test]
    fn delete_one() {
        let temp_dir = PathBuf::new()
            .join("./temp/delete_one");
        if temp_dir.exists() {
            std::fs::remove_dir_all(temp_dir.clone())
                .expect("Can NOT delete temp dir!!");
        }
        let db_path = temp_dir.join("test.db");

        // This will panic and exit the program if an error occurs.
        super::register("test", db_path.clone());
        super::delete_one(db_path.clone(), "1".to_string());
        let logs = super::get_logs(db_path.clone(), 0, "".to_string());
        assert_eq!(logs.len(), 0, "Can NOT delete the log!!");

        std::fs::remove_dir_all(temp_dir.clone())
            .expect("Can NOT delete temp dir!!");
    }
}