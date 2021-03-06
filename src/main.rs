use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
pub fn lines_from_file<T: AsRef<Path>>(filename: T) -> String {
    let file = std::fs::File::open(&filename);
    let file = match file {
        Ok(n) => n,
        Err(_) => {
            println!("Error! File not found!");
            exit(0);
        }
    };
    return std::fs::read_to_string(&filename).unwrap();
}
fn get_file<T: AsRef<Path>>(filename: T) -> std::fs::File {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename);
    let file = match file {
        Ok(n) => n,
        Err(_) => {
            println!("Error! File not found!");
            exit(0);
        }
    };
    return file;
}
#[allow(unused_must_use)]
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let mut file = get_file(&args.path);
    enable_raw_mode().unwrap();
    loop {
        let event = read().unwrap();
        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        } else if event == Event::Key(KeyCode::Backspace.into()) {
            let contents = lines_from_file(&args.path);
            std::fs::write(&args.path, &contents[0..contents.len() - 1]);
        } else {
            for i in b' '..b'~' {
                if event == Event::Key(KeyCode::Char(i as char).into()) {
                    &file.write(&[i]);
                }
            }
        }
    }
    disable_raw_mode().unwrap();
    Ok(())
}
