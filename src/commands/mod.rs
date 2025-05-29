use clap::{arg, Command};

pub fn commands() -> Vec<Command> {
    vec![
        Command::new("password-manager")
            .alias("pm")
            .about("Create, save, update, delete, and manage passwords.")
            .subcommands([
                Command::new("generate")
                    .alias("g")
                    .about("Generate new password as ASCII/HEX.")
                    .args([
                        arg!([LENGTH]             "Password length (e.g. 128)."),
                        arg!(--hex                "Password as hexadecimal."),
                        arg!(--"no-symbols"       "Password as letters and numbers (a-z, A-Z, 1-0)."),
                        arg!(--"add-set" <STRING> "Add custom set to the samples."),
                        arg!(--custom    <STRING> "Generate from custom samples."),
                        arg!(--save      <NAME>   "Save the password (e.g. \"mohaned2023 github\").")
                    ]),
                Command::new("save")
                    .about("Save custom password in the database.")
                    .arg(arg!(<NAME> "Password name (e.g. \"mohaned2023 github\").")),
                Command::new("find")
                    .about("Search for password in the database.")
                    .arg(arg!(<STRING> "String in the password name (e.g. \"github\").")),
                Command::new("show")
                    .about("Display all passwords in the database.")
                    .arg(arg!(-t --table "Show as table.")),
                Command::new("count")
                    .about("Get the number of passwords you saved in the database."),
                Command::new("update")
                    .alias("up")
                    .about("Update the password information.")
                    .args([
                        arg!(<ID>          "Password id (e.g. 23)."),
                        arg!(-n --name     "Update the name."),
                        arg!(-p --password "Update the password.")
                    ]),
                Command::new("delete")
                    .about("Delete specific password from the database.")
                    .arg(arg!(<ID> "Password id (e.g. 23).")),
                Command::new("encrypt")
                    .alias("en")
                    .about("Encrypt the password manager database.")
                    .args([
                        arg!(--key    "Use custom key.")
                    ]),
                Command::new("decrypt")
                    .alias("de")
                    .about("Decrypt the password manager database."),
            ]),
        Command::new("encryption-manager")
            .alias("em")
            .about("Encrypt/Decrypt file and folder and encode/decode strings.")
            .subcommands([
                Command::new("encrypt-file")
                    .alias("enf")
                    .about("Encrypt file.")
                    .args([
                        arg!(<PATH>   "File path (e.g. \"/home/user/important/image.png\")."),
                        arg!(--key    "Use custom key."),
                        arg!(--delete "Delete the origin file.")
                    ]),
                Command::new("decrypt-file")
                    .alias("def")
                    .about("Decrypt file.")
                    .args([
                        arg!(<PATH>   "File path (e.g. \"/home/user/important/image.png.x\")."),
                        arg!(--delete "Delete the origin file."),
                        arg!(--xpmv1  "Decrypt XPManager v1.0 file.")
                    ]),
                Command::new("encrypt-dir")
                    .alias("end")
                    .about("Encrypt directory.")
                    .args([
                        arg!(<PATH>         "Directory path (e.g. \"/home/user/important\")."),
                        arg!(--delete       "Delete the origin files in the directory."),
                        arg!(--"no-threads" "Encrypt directory using the main thread only."),
                        arg!(--key          "Use custom key.")
                    ]),
                Command::new("decrypt-dir")
                    .alias("ded")
                    .about("Decrypt directory.")
                    .args([
                        arg!(<PATH>         "Directory path (e.g. \"/home/user/important\")."),
                        arg!(--delete       "Delete the origin files in the directory."),
                        arg!(--"no-threads" "Decrypt directory using the main thread only."),
                        arg!(--xpmv1        "Decrypt XPManager v1.0 directory.")
                    ]),
                Command::new("encode")
                    .alias("enc")
                    .about("Encode strings using different techniques.")
                    .args([
                        arg!(--xpmv1      "XPManager v1.0 key technique."),
                        arg!(--hex        "Hexadecimal."),
                        arg!(--"hex-hash" "Hash using hexadecimal."),
                        arg!(--bin        "Binary.")
                    ]),
                Command::new("decode")
                    .alias("dec")
                    .about("Decode strings using different techniques.")
                    .args([
                        arg!(--xpmv1 "XPManager v1.0 key technique."),
                        arg!(--hex   "Hexadecimal."),
                        arg!(--bin   "Binary.")
                    ])
            ]),
        Command::new("backup-manager")
            .alias("bm")
            .about("Create/Restore backup for passwords/logs database.")
            .subcommands([
                Command::new("backup")
                    .about("Create backup for passwords/logs database.")
                    .args([
                        arg!(<PATH>     "The directory you want to save to (e.g. \"/home/user/backup\")."),
                        arg!(--password "Password manager database."),
                        arg!(--log      "Log manager database.")
                    ]),
                Command::new("restore")
                    .about("Restore passwords/logs database.")
                    .args([
                        arg!(<PATH>            "Restore file path (e.g. \"/home/user/backup/data.x\")."),
                        arg!(--password        "Password manager database."),
                        arg!(--xpmv1           "XPManager v1.0 database."),
                        arg!(--"password-json" "Restore from a custom JSON file."),
                        arg!(--log             "Log manager database.")
                    ])
            ]),
        Command::new("log-manager")
            .alias("lm")
            .about("Show, find, delete, or clear xpm logs.")
            .subcommands([
                Command::new("clear")
                    .about("Clear all logs."),
                Command::new("show")
                    .about("Display all/some logs.")
                    .args([
                        arg!(-l --length <NUMBER> "Number of logs (e.g. 20).")
                    ]),
                Command::new("find")
                    .about("Find logs based on date/string.")
                    .subcommands([
                        Command::new("date")
                            .about("Search using the date")
                            .args([
                                arg!(--year  <YEAR>  "Year number (e.g. 2025)."),
                                arg!(--month <MONTH> "Month number (e.g. 3)."),
                                arg!(--day   <DAY>   "Day number (e.g. 29).")
                            ]),
                        Command::new("string")
                            .about("Search using string")
                            .arg(arg!(<STRING> "The search string (e.g. \"github\")."))
                    ]),
                Command::new("delete")
                    .about("Delete single log by id.")
                    .args([
                        arg!(<ID>   "Log id.")
                    ])
            ]),
    ]
}