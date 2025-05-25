
use crate::{
    errorlib, 
    loglib
};
use colored::Colorize;
use rand::seq::{IndexedRandom, IteratorRandom};
use std::path::PathBuf;

/// The password sample types.
/// uses to chooes sample to generate the password.
#[derive(PartialEq)]
pub enum PasswordSample {
    Ascii,
    NoSymbols,
    Hex
}

/// Generate sample based on the type.
/// types from PasswordSample enum:
/// - Ascii: A-Z, a-z, 0-9 and some symbols.
/// - NoSymbols: A-Z, a-z and 0-9.
/// - Hex: 0-9 and A-F.
/// 
/// ### Example:
/// ```
/// let sample = utilities::get_sample(
///     utilities::PasswordSample::Hex
/// );
/// let hex: Vec<char> = ('0'..='9')
///     .chain('A'..='F')
///     .collect();
/// assert_eq!(sample, hex);
/// ```
pub fn get_sample(sample: PasswordSample) -> Vec<char> {
    match sample {
        PasswordSample::Ascii => return ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .chain([
                '!', '@', '#', '$', '%', '^', '&', '(', ')', '-', '+', '=', '~',
                '[', ']', '{', '}', '/', '|', ':', ';', '?', ',', '.', '<', '>'
            ])
            .collect(),
        PasswordSample::NoSymbols => return ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .collect(),
        PasswordSample::Hex => return ('0'..='9')
            .chain('A'..='F')
            .collect()
    }
}

/// Generate random number as `String` between 32 to 73.
/// 
/// ## Example:
/// ```
/// let number_str = utilities::get_ran_string_number();
/// let number = number_str.parse::<i32>().unwrap_or(0);
/// assert_eq!(number >= 32 && number <= 72, true);
/// ```
pub fn get_ran_string_number() -> String {
    let mut rag = rand::rng();
    // Choose random length between 32 to 72.
    return (32..=72)
        .choose(&mut rag)
        .unwrap()
        .to_string();
}

/// Read a string from standard input.
/// 
/// ### Example
/// ```
/// let input = utilities::input("Enter your name: ");
/// if input == "Mohaned" {
///     println!("Hello Mohaned");
/// } else {
///     println!("hi {}", input);
/// }
/// ```
pub fn input(message: &str) -> String {
    use std::io::Write;
    print!("{}", message);
    std::io::stdout().flush().expect("Flush Error!");
    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Read line Error!");
    return line.trim().to_owned();
}

/// Get user confirmation by entring 6-digit code.
/// Exit the program if the confirmation fails.
/// 
/// ### Example:
/// ```
/// let mut logger = loglib::Logger::new("delete-file");
/// logger.warning("file will be deleted!");
/// 
/// utilities::confirm(); // Waiting for the user...
/// logger.start(); // Reset the counter, confirmation will take a while.
/// 
/// delete_file(PathBuf::new().join("./file.txt"));
/// ```
pub fn confirm() {
    let mut logger = loglib::Logger::new("confirm");
    logger.warning("This process requires confirmation!");
    let mut rng = rand::rng();
    let sample: Vec<char> = ('1'..='9').collect();
    let mut confirmation = String::new();
    for _ in 0..6 {
        confirmation.push(
            sample.choose(&mut rng)
                .unwrap()
                .clone()
        );
    }
    let value = input(
        &format!("Please enter {} to continue: ", confirmation.green())
    );
    logger.start();
    if value != confirmation {
        logger.error(
            "This process stopped, confirmation error",
            errorlib::ExitErrorCode::ConfirmationNotMatch
        )
    }
    logger.info("confirmation completed successfully.");
}

/// Distribute paths to `Vec<Vec<PathBuf>>` based on the threads number,
/// every `Vec<PathBuf>` carries all paths for one thread, the length
/// of `Vec<Vec<PathBuf>>` is the number of threads.
/// 
/// ### Example:
/// ```
/// let files_paths = vec!["file-1.txt", "file-2.txt", "file-3.txt", "file-4.txt"];
/// let let distributed_paths = utilities::distribute_paths(files_paths.clone());
/// // if the number of theards >= 4
/// assert_eq!(
///     distributed_paths, 
///     vec![
///         vec!["file-1.txt"],
///         vec!["file-2.txt"],
///         vec!["file-3.txt"],
///         vec!["file-4.txt"],
///     ]
/// );
/// ```
pub fn distribute_paths(files_paths: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut paths: Vec<Vec<PathBuf>> = Vec::new();
    let thread_num = num_cpus::get().max(1);
    if files_paths.len() <= thread_num {
        for i in files_paths.clone() {
            paths.push(vec![i]);
        }
    } else {
        let items_num_per_thread = (files_paths.len() as f64/thread_num as f64).ceil() as usize;
        let mut seek = 0;
        for i in 1..=thread_num {
            if items_num_per_thread*i > files_paths.len() {
                paths.push(
                    (files_paths[seek..files_paths.len()]).to_vec()
                );
                break;
            }
            paths.push(
                (files_paths[seek..items_num_per_thread*i]).to_vec()
            );
            seek = items_num_per_thread * i;
        }
    }
    return paths;
}



#[cfg(test)]
mod tests {
    #[test]
    fn get_sample() {
        let mut sample = super::get_sample(super::PasswordSample::Ascii);
        let ascii: Vec<char>= ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .chain([
                '!', '@', '#', '$', '%', '^', '&', '(', ')', '-', '+', '=', '~',
                '[', ']', '{', '}', '/', '|', ':', ';', '?', ',', '.', '<', '>'
            ])
            .collect();
        let hex: Vec<char> = ('0'..='9')
            .chain('A'..='F')
            .collect();
        let no_sympols: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .collect();
        assert_eq!(sample, ascii, "ASCII samples NOT match!!");
        sample = super::get_sample(super::PasswordSample::Hex);
        assert_eq!(sample, hex, "HEX samples NOT match!!");
        sample = super::get_sample(super::PasswordSample::NoSymbols);
        assert_eq!(sample, no_sympols, "NoSymbols samples NOT match!!");
    }

    #[test]
    fn get_ran_string_number() {
        let number_str = super::get_ran_string_number();
        let number = number_str
            .parse::<u8>()
            .expect("Can NOT parse the random number to u8!!");
        assert!(number >= 32 && number <= 72, "Random number NOT in (32 <= x <= 72)!!");
    }

}