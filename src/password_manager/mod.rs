pub mod generate;
pub mod save;
pub mod find;
pub mod show;
pub mod update;
pub mod count;
pub mod delete;
pub mod encrypt;
pub mod decrypt;

use clap::ArgMatches;

/// Encrypt and decrypt password manager database.
pub struct PMDatabaseEncrption {
    en_path: String,
    de_path: String,
    key: String
}

impl PMDatabaseEncrption {
    /// Creates a new instance of a `PMDatabaseEncrption`.
    /// It will get the password manager database paths from
    /// `filelib::pm`. 
    /// 
    /// ### Example:
    /// ```
    /// PMDatabaseEncrption::new();
    /// ```
    pub fn new() -> PMDatabaseEncrption {
        PMDatabaseEncrption { 
            en_path: crate::filelib::pm::get_encrypted_db_path()
                .to_str()
                .unwrap()
                .to_string(),
            de_path: crate::filelib::pm::get_decrypted_db_path()
                .to_str()
                .unwrap()
                .to_string(),
            key: "".to_owned()
        }
    }

    /// Set the key. It will take the key from the user 
    /// using `utilities::input` if key is None, 
    /// else will use the input key.
    /// 
    /// ### Example:
    /// ```
    /// let mut pm_db = PMDatabaseEncrption::new();
    /// pm_db.set_key(None); // It will ask the user to enter the key.
    /// ```
    pub fn set_key(&mut self, key: Option<String>) {
        if key == None {
            self.key = crate::utilities::input("Enter the key: ");
        } else {
            self.key = key.unwrap();
        }
    }

    /// Decrypt the password manager database.
    /// 
    /// ### Example:
    /// ```
    /// let mut pm_db = PMDatabaseEncrption::new();
    /// pm_db.decrypt();
    /// ```
    pub fn decrypt(&mut self) {
        self.set_key(None);
        crate::encryption_manager::decrypt_file::decrypt(
            self.en_path.clone(),
            self.key.clone()
        );
        crate::filelib::wipe_delete(self.en_path.clone());
    }

    /// Encrypt the password manager database.
    /// 
    /// ### Example:
    /// ```
    /// let pm_db = PMDatabaseEncrption::new();
    /// pm_db.encrypt();
    /// ```
    pub fn encrypt(&self) {
        crate::encryption_manager::encrypt_file::encrypt(
            self.de_path.clone(), 
            self.key.clone()
        );
        crate::filelib::wipe_delete(self.de_path.clone());
    }
}