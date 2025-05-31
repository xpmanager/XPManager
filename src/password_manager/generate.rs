use super::{
    ArgMatches,
    PMDatabaseEncrption
};
use rand::seq::{
    IndexedRandom, 
    SliceRandom
};
use crate::{
    errorlib,
    filelib,
    utilities,
    loglib,
    dblib,
    displaylib
};

/// Generate password from a sample.
/// 
/// ### Example:
/// ```
/// let password = generate(
///     128, // the length
///     utilities::get_sample( // the sample
///         utilities::PasswordSample::Hex // the sample type
///     )
/// );
/// assert_eq!(password.len(), 128);
/// println!("Your password: {}", password);
/// ```
fn generate(length: u16, sample: &mut Vec<char> ) -> String {
    let mut rng = rand::rng();
    sample.shuffle(&mut rng);
    let mut password: String = String::new();
    for _ in 0..length {
        password.push(
            sample.choose(&mut rng)
                .unwrap()
                .clone()
        );
    }
    password
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("generate-password");

    // Get the length
    let length = command
        .get_one::<String>("LENGTH")
        .unwrap_or(&utilities::get_ran_string_number())
        .parse::<u16>();
    if length.is_err() {
        logger.error(
            &format!("<LENGTH> must be unsigned integer from 0 to {}!", u16::MAX),
            errorlib::ExitErrorCode::Input
        )
    }

    // generate the sample based on the type
    let mut sample = if *command.get_one::<bool>("hex").unwrap_or(&false) {
        utilities::get_sample(utilities::PasswordSample::Hex)
    } else if *command.get_one::<bool>("no-symbols").unwrap_or(&false) {
        utilities::get_sample(utilities::PasswordSample::NoSymbols)
    } else {
        utilities::get_sample(utilities::PasswordSample::Ascii)
    };

    // Custom sample or add a custom set to the sample.
    let add_set = command.get_one::<String>("add-set");
    let custom = command.get_one::<String>("custom");
    if add_set.is_some() || custom.is_some() {
        let mut _new_sample;
        if let Some(sample_str) = custom {
            if sample_str.len() > 1000 {
                logger.error(
                    "sample length greater than 1000!!",
                    errorlib::ExitErrorCode::Input
                );
            }
            sample = Vec::new();
            _new_sample = sample_str.chars();
        } else {
            _new_sample = add_set.unwrap().chars();
        }
        for c in _new_sample {
            // User shouldn't have space in the password.
            if c == ' ' {
                logger.error(
                    "the sample contain space!!", 
                    errorlib::ExitErrorCode::SampleContainSpace
                );
            }
            sample.push(c);
        }
    }

    // generate the password from the sample
    let mut _password: String = generate(
        length.unwrap(),
        &mut sample
    );
    logger.info("password generated successfully.");
    
    // save the password
    if let Some(password_name) = command.get_one::<String>("save") {
        let pm_db_state = filelib::pm::db_state();
        let mut pm_db_encryption = PMDatabaseEncrption::new();
        let mut _is_db_decrypted: bool = false;
        let pm_decrypted_path = filelib::pm::get_decrypted_db_path();
        if pm_db_state == filelib::FileState::NotFound {
            filelib::create_file(pm_decrypted_path.clone());
            dblib::pm::create_passwords_table(pm_decrypted_path.clone());
            dblib::log::register("create passwords table", filelib::log::get_log_db_path());
        } else if pm_db_state == filelib::FileState::Encrypted {
            logger.warning("database is encrypted!");
            pm_db_encryption.decrypt();
            logger.start();
            _is_db_decrypted = true;
            logger.info("password manager database decrypted successfully.");
        }
        dblib::pm::save_password(
            pm_decrypted_path,
            password_name.clone(),
            _password.clone()
        );
        dblib::log::register(
            &format!("'{}' saved successfully.", password_name), 
            filelib::log::get_log_db_path()
        );
        if _is_db_decrypted {
            pm_db_encryption.encrypt();
            logger.info("password manager database encrypted successfully.");
        }
    }
    displaylib::passwords::display_one(_password);
}


#[cfg(test)]
mod tests {
    #[test]
    fn generate_password() {
        let password = super::generate(
            512, 
            &mut super::utilities::get_sample(
                super::utilities::PasswordSample::Ascii
            )
        );
        assert_eq!(password.len(), 512, "ASCII password length is NOT 512!!");
    }
}