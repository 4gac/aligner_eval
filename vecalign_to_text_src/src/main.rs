use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use clap::{arg, command, crate_authors, crate_version, Command, Parser};

struct Sentences {
    en: Vec<u32>,
    sk: Vec<u32>,
}

impl Sentences {
    fn new() -> Sentences {
        Sentences {
            en: Vec::new(),
            sk: Vec::new(),
        }
    }

    fn push_to_en(&mut self, number: u32) {
        self.en.push(number);
    }

    fn push_to_sk(&mut self, number: u32) {
        self.sk.push(number);
    }
}
#[derive(Debug)]
enum VecAlignError {
    ColumnOneMissing(String),
    ColumnTwoMissing(String),
}

fn main() {
    let matches = Command::new("Vecalign to text")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(
            "creates aligned text files from given source and target based on vecaligner results",
        )
        .arg(arg!(--source <VALUE>).required(true).short('s'))
        .arg(arg!(--target <VALUE>).required(true).short('t'))
        .arg(arg!(--alignment <VALUE>).required(true).short('a'))
        .get_matches();

    let source = matches
        .get_one::<String>("source")
        .expect("required source file argument");
    let target = matches
        .get_one::<String>("target")
        .expect("required target file argument");
    let alignment = matches
        .get_one::<String>("alignment")
        .expect("required alignment file argument");

    let mut source_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(source)
        .expect("Unable to open file");
    let mut target_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(target)
        .expect("Unable to open file");
    let mut aligment_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(alignment)
        .expect("Unable to open file");

    let mut source_contents = String::new();
    source_file.read_to_string(&mut source_contents).unwrap();
    let mut target_contents = String::new();
    target_file.read_to_string(&mut target_contents).unwrap();
    let mut alignment_contents = String::new();
    aligment_file
        .read_to_string(&mut alignment_contents)
        .unwrap();

    match create_sentences(&alignment_contents) {
        Err(e) => match e {
            VecAlignError::ColumnOneMissing(err) => println!("{}", err),
            VecAlignError::ColumnTwoMissing(err) => println!("{}", err),
        },
        Ok(sentences) => {
            let res = align_files(&source_contents, &target_contents, &sentences);
            write_alignments(&res.0, &source, &res.1, &target);
            println!("Vecaligned files created");
        }
    }
}

fn create_sentences(alignment_contents: &str) -> Result<Sentences, VecAlignError> {
    let mut sent = Sentences::new();

    for line in alignment_contents.lines() {
        if line.is_empty() {
            continue;
        }

        let mut columns = line.split(":");

        match columns.next() {
            Some(column) => {
                let number_of_entries = column.split(",").count();
                sent.push_to_en(number_of_entries as u32);
            }
            None => {
                return Err(VecAlignError::ColumnOneMissing(String::from(
                    "Cannot find alignment indexes for source sentences",
                )));
            }
        }

        match columns.next() {
            Some(column) => {
                let number_of_entries = column.split(",").count();
                sent.push_to_sk(number_of_entries as u32);
            }
            None => {
                return Err(VecAlignError::ColumnTwoMissing(String::from(
                    "Cannot find alignment indexes for target sentences",
                )));
            }
        }
    }

    Ok(sent)
}

fn align_files(
    source_contents: &str,
    target_contents: &str,
    sentences: &Sentences,
) -> (String, String) {
    let mut source_lines: Vec<&str> = source_contents.split("\n").collect();
    let mut target_lines: Vec<&str> = target_contents.split("\n").collect();

    let mut align_en = String::new();
    let mut align_sk = String::new();

    for i in sentences.en.iter() {
        for y in 0..*i as usize {
            if y > 0 {
                align_en.push(' ');
            }

            align_en.push_str(source_lines.first().expect("failed to take"));

            source_lines.remove(0);
        }
        align_en.push_str("\n");
    }

    for i in sentences.sk.iter() {
        for y in 0..*i as usize {
            if y > 0 {
                align_sk.push(' ');
            }

            align_sk.push_str(target_lines.first().expect("failed to take"));

            target_lines.remove(0);
        }
        align_sk.push_str("\n");
    }
    return (align_en, align_sk);
}

fn write_alignments(source: &str, source_dest: &str, target: &str, target_dest: &str) {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(source_dest.to_owned() + "_vecaligned.txt")
        .expect("Unable to open file");

    f.write_all(source.as_bytes())
        .expect("Unable to write data");

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(target_dest.to_owned() + "_vecaligned.txt")
        .expect("Unable to open file");

    f.write_all(target.as_bytes())
        .expect("Unable to write data");
}
