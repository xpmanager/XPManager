use clap::Command;

mod password_manager;
mod encryption_manager;
mod log_manager;
mod backup_manager;
mod commands;
mod matcheslibs;
mod utilities;
mod loglib;
mod dblib;
mod displaylib;
mod filelib;
mod errorlib;

fn main() {
    let matches = Command::new("xpm")
        .about("Password manager, File/Folder encryptor, Strings encoder.")
        .version("2.3.0")
        .author("Mohaned Sherhan")
        .subcommands(commands::commands())
        .get_matches();
    matcheslibs::matches(matches);
    filelib::pm::warning_encrypt_database();
}
