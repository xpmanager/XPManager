use super::{
    ArgMatches,
    Fernet,
    Write,
    Read
};
use crate::{
    errorlib,
    filelib, 
    loglib,
    utilities,
    dblib
};

pub fn xpmv1_decryption(path: String, key: String) {
    // In the XPManager v1.0 we do not use buffer to read
    // from the file, So the encryption/decryption 
    // will be different.
    let logger = loglib::Logger::new("xpmv1-decryption");
    if let Ok(mut en_file) = std::fs::File::open(&path) {
        if let Ok(mut de_file) = std::fs::File::create(
            filelib::make_decrypt_path(path)
        ) {
            if let Some(fernet) = Fernet::new(&key) {
                let mut en_data: String = String::new(); 
                if en_file.read_to_string(&mut en_data).is_err() {
                    logger.error(
                        "can NOT read the file!",
                        errorlib::ExitErrorCode::FileRead
                    )
                }
                if let Ok(de_data) = fernet.decrypt(&en_data) {
                    de_file.write_all(&de_data).unwrap();
                    return;
                } else {
                    logger.error(
                        "it's look likes your encryption data is a broken!",
                        errorlib::ExitErrorCode::InvalidEncryptionData
                    )
                }
            }
            logger.error("key error!", errorlib::ExitErrorCode::InvalidKey);
        }
    }
    logger.error("can NOT open the file!", errorlib::ExitErrorCode::FileOpen);
}

pub fn decrypt(path: String, key: String) {
    let logger = loglib::Logger::new("decrypt-file");
    if let Some(fernet) = Fernet::new(&key) {
        if let Ok(mut en_file) = std::fs::File::open(&path) {
            if let Ok(mut de_file) = std::fs::File::create(
                filelib::make_decrypt_path(path)
            ) {
                // The encryption block size buffer
                // Uses to read the size of the block
                let mut size_buf = [0u8; 4];
                loop {
                    // Read the size of the block before reading the block
                    if en_file.read_exact(&mut size_buf).is_err() {
                        break;
                    }
                    // Encryption file format: "<length><en-data><length><en-data>"
                    // Read the block size and make a buffer with this size and then
                    // read the encryption block using the buffer.
                    let size = u32::from_be_bytes(size_buf) as usize;
                    let mut encryption_buffer = vec![0u8; size];
                    en_file.read_exact(&mut encryption_buffer).unwrap();
                    let data = fernet.decrypt(
                        &String::from_utf8(
                            encryption_buffer
                        ).unwrap()
                    ).unwrap();
                    // Save the decrypted blocks one by one.
                    de_file.write_all(&data).unwrap();
                }
                return;
            }
        }
        logger.error(
            "can NOT open the file!", 
            errorlib::ExitErrorCode::FileOpen
        );
    } else {
        logger.error(
            "key error!", 
            errorlib::ExitErrorCode::InvalidKey
        );
    }
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-file");
    let path= command.get_one::<String>("PATH").unwrap();
    let file_state = filelib::get_file_state(path.clone());
    if file_state == filelib::FileState::NotFound {
        logger.error(
            "file NOT found!",
            errorlib::ExitErrorCode::FileNotFound
        );
    } else if file_state == filelib::FileState::Decrypted {
        logger.error(
            "file NOT encrpted!",
            errorlib::ExitErrorCode::FileNotEncrypted
        );
    }
    let key =  utilities::input("Enter your key: ");
    logger.start();
    logger.info("decryption in progress....");
    if *command.get_one::<bool>("xpmv1").unwrap_or(&false) {
        logger.warning("do not use --xpmv1 with the XPManager v2.0 encryption it will break your file!!");
        logger.warning("XPManager v1.0 can not handle large files!!");
        xpmv1_decryption(path.clone(), key);
    } else {
        decrypt(path.clone(), key);
    }
    logger.info("file decrypted successfully.");
    dblib::log::register(
        &format!("file '{}' encrypted", path.clone()),
        filelib::log::get_log_db_path()
    );
    if *command.get_one::<bool>("delete").unwrap_or(&false) {
        logger.start();
        filelib::wipe_delete(path.clone());
        logger.info("file wiped and deleted successfully.");
        dblib::log::register(
            &format!("file '{}' wiped", path),
            filelib::log::get_log_db_path()
        );
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::filelib::{create_file, delete_file};

    #[test]
    fn decrypt() {
        let temp_dir = PathBuf::new()
            .join("./temp/decrypt");
        let file = temp_dir.join("test.txt.x");
        let de_file = temp_dir.join("test.txt");
        create_file(file.clone());
        assert_eq!(file.exists(), true, "Can NOT create the test file!!");
        let file_path_str = file
            .to_str()
            .expect("Can NOT parse PathBuf to &str!!")
            .to_string();

        delete_file(de_file.clone());
        assert_eq!(de_file.exists(), false, "Can NOT delete the test file!!");
        let key = super::Fernet::generate_key();
        super::decrypt(file_path_str.clone(), key.clone());
        assert_eq!(de_file.exists(), true, "Can NOT decrypt the test file!!");

        std::fs::remove_dir_all(temp_dir)
            .expect("Can NOT delete temp files!!");
    }
}