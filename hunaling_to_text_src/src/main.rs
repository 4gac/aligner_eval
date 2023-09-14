use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use clap::{arg, crate_authors, crate_version, Command};

fn main() {
    let matches = Command::new("Vecalign to text")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("creates aligned text files from given source based on hunalign results")
        .arg(arg!(--source <VALUE>).required(true).short('s'))
        .get_matches();

    let source = matches
        .get_one::<String>("source")
        .expect("cannot open file");
    let mut aligment_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(source)
        .expect("Unable to open file");

    let mut source_contents = String::new();
    aligment_file.read_to_string(&mut source_contents).unwrap();

    create_algined_files(&source_contents, &source);
}

fn create_algined_files(s: &str, dest: &str) {
    let lines: Vec<&str> = s.split("\n").collect();

    let mut align_en = String::new();
    let mut align_sk = String::new();

    for line in lines {
        if line.is_empty() {
            break;
        }
        let line = &line.replace(" ~~~ ", " ");
        let splitted: Vec<&str> = line.split("\t").collect();

        align_sk.push_str(splitted[0]);
        align_sk.push_str("\n");
        align_en.push_str(splitted[1]);
        align_en.push_str("\n");
    }

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(dest.to_string() + "_hunalign_sk.txt")
        .expect("Unable to open file");

    f.write_all(align_sk.as_bytes())
        .expect("Unable to write data");

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(dest.to_string() + "_hunalign_en.txt")
        .expect("Unable to open file");

    f.write_all(align_en.as_bytes())
        .expect("Unable to write data");
}
