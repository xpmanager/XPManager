use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use super::{
    decrypt_file, 
    ArgMatches,
    PathBuf
};
use crate::{
    displaylib, 
    filelib, 
    loglib, 
    utilities,
    dblib
};


/// Decrypt list of files.
/// 
/// ### Example:
/// ```
/// let files: Vec<PathBuf> = vec![
///     PathBuf::new().join('/folder/to/file-1.txt.x'),
///     PathBuf::new().join('/folder/to/file-2.txt.x'),
///     PathBuf::new().join('/folder/to/file-3.txt.x')
/// ];
/// let decryption_key = "<your-key>".to_string();
/// let delete_files = false;
/// let is_xpmv1_files = false;
/// 
/// decrypt(&files, decryption_key, delete_files, is_xpmv1_files);
/// ```
fn decrypt(paths: &Vec<PathBuf>, key: String, is_delete: bool, is_xpmv1: bool) {
    let mut logger = loglib::Logger::new("decrypt-dir-thread");
    for file in paths {
        logger.start();
        let file_path_string = file.to_str().unwrap().to_owned();
        if filelib::get_file_state(
            file_path_string.clone()
        ) == filelib::FileState::Decrypted {
            logger.warning(
                &format!("file not encrypted '{}'?", file_path_string)
            );
            continue;
        }
        if is_xpmv1 {
            decrypt_file::xpmv1_decryption(
                file_path_string.clone(),
                key.clone()
            );
        } else {
            decrypt_file::decrypt(
                file_path_string.clone(),
                key.clone()
            );
        }
        logger.info(
            &format!("decrypted '{}'.", file.display())
        );
        dblib::log::register(
            &format!("file '{}' decrypted", file.display()),
            filelib::log::get_log_db_path()
        );
        if is_delete {
            filelib::wipe_delete(file_path_string.clone());
            logger.info("file was wiped successfully.");
            dblib::log::register(
                &format!("file '{}' wiped", file.display()),
                filelib::log::get_log_db_path()
            );
        }
    }
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decrypt-dir");
    let path= command.get_one::<String>("PATH").unwrap();
    let mut files_paths: Vec<PathBuf> = vec![];
    let is_xpmv1 = *command.get_one::<bool>("xpmv1").unwrap_or(&false);
    let is_delete = *command.get_one::<bool>("delete").unwrap_or(&false);
    let no_threads = *command.get_one::<bool>("no-threads").unwrap_or(&false);
    let key = utilities::input("Enter your key: ");
    logger.start();
    filelib::dir_files_tree(
        PathBuf::new().join(path), 
        &mut files_paths
    );
    if is_delete {
        logger.warning(
            &format!(
                "you are about to decrypt and {} all encryption files in this directory '{}'",
                "delete".red(),
                path.clone()
            )
        );
        utilities::confirm();
        logger.start();
    }
    logger.info("directory listed successfully.");
    
    if no_threads {
        logger.info("start the decryption using the main thread.");
        decrypt(
            &files_paths, 
            key.clone(), 
            is_delete, 
            is_xpmv1
        );
    } else {
        logger.info("start the decryption with the max number of threads.");
        // Distribute files over the number of threads 
        let distributed_paths: Vec<Vec<PathBuf>> = utilities::distribute_paths(files_paths.clone());
    
        // Run the threads
        distributed_paths.par_iter().for_each(|paths| {
            decrypt(
                paths, 
                key.clone(), 
                is_delete, 
                is_xpmv1
            );
        });
    }

    logger.info("directory decrypted successfully.");
    displaylib::key::display(key);
}