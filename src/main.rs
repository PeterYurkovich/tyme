use chrono::offset;
use homedir::my_home;
use std::{env, fs, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let nonfunction_args = &args[1..];
    let note_text = nonfunction_args.join(" ");

    let home_dir = my_home()
        .expect("User is expected to have a home directory")
        .unwrap();

    let config_directory = home_dir.as_path().join(".config/tyme/");
    fs::create_dir_all(&config_directory.as_os_str())
        .expect("Error creating tyme config directory");
    let output_file = config_directory.join("tyme.log");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .unwrap();

    let timestamp = offset::Local::now().timestamp();
    if let Err(e) = writeln!(file, "{} | {}", timestamp, note_text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
