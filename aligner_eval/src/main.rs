use std::{
    env::args,
    fs::{File, OpenOptions},
    io::Read,
};

use clap::{arg, crate_authors, crate_version, Command};

struct Alignment {
    pairs: Vec<(String, String)>,
}

impl Alignment {
    fn new() -> Alignment {
        Alignment { pairs: Vec::new() }
    }

    fn create_alignemnt(&mut self, f1: &mut File, f2: &mut File) {
        let mut source_contents = String::new();
        f1.read_to_string(&mut source_contents).unwrap();
        let mut target_contents = String::new();
        f2.read_to_string(&mut target_contents).unwrap();

        for it in source_contents
            .split("\n")
            .into_iter()
            .zip(target_contents.split("\n").into_iter())
        {
            let (s, t) = it;
            self.pairs.push((s.to_string(), t.to_string()))
        }
    }
}

#[derive(Debug)]
enum EvaluateError {
    CannotFindSourceRef(String),
}

struct BadAlignInfo {
    line_number: u32,
    text: String,
}

struct Report {
    alignments_total: u32,
    alignments_matched: u32,
    alignments_missmatched_source: u32,
    alignments_missmatched_target: u32,

    bad_sources: Vec<BadAlignInfo>,
    bad_targets: Vec<BadAlignInfo>,
}

impl Report {
    fn new(
        total: u32,
        matched: u32,
        miss_source: u32,
        miss_target: u32,
        bad_sources: Vec<BadAlignInfo>,
        bad_targets: Vec<BadAlignInfo>,
    ) -> Report {
        Report {
            alignments_total: total,
            alignments_matched: matched,
            alignments_missmatched_source: miss_source,
            alignments_missmatched_target: miss_target,
            bad_sources,
            bad_targets,
        }
    }

    // precision
    fn precision(&self) -> f32 {
        self.alignments_matched as f32 / self.alignments_total as f32
    }
}

fn evaluate_alignment(align: &Alignment, align_ref: &Alignment) -> Result<Report, EvaluateError> {
    let mut total_num_of_alignments = 0;
    let mut matched = 0;
    let mut bad_source_align = 0;
    let mut bad_target_align = 0;
    let mut bad_sources: Vec<BadAlignInfo> = Vec::new();
    let mut bad_targets: Vec<BadAlignInfo> = Vec::new();

    for (i, it) in align.pairs.iter().enumerate() {
        total_num_of_alignments += 1;
        let (s, t) = it;
        // println!("{}", i);

        let found = align_ref
            .pairs
            .iter()
            .find(|sentence| s.to_owned() == sentence.0);

        // found match in english texts
        if let Some((ref_s, ref_t)) = found {
            // compare with slovak ref
            if ref_t == t {
                matched += 1;
            }
            // possible diff in slovak texts
            else {
                bad_target_align += 1;
                bad_targets.push(BadAlignInfo {
                    line_number: i as u32 + 1,
                    text: t.to_owned(),
                })
            }
        }
        // possible diff in english texts
        else {
            bad_source_align += 1;
            bad_sources.push(BadAlignInfo {
                line_number: i as u32 + 1,
                text: s.to_owned(),
            })
        }
    }

    Ok(Report::new(
        total_num_of_alignments,
        matched,
        bad_source_align,
        bad_target_align,
        bad_sources,
        bad_targets,
    ))
}

fn main() {
    let matches = Command::new("Aligner evaluation")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(arg!(--source <VALUE>).required(true).short('s'))
        .arg(arg!(--target <VALUE>).required(true).short('t'))
        .arg(arg!(--source_ref <VALUE>).required(true))
        .arg(arg!(--target_ref <VALUE>).required(true))
        .arg(arg!(--verbose))
        .get_matches();

    let source = matches
        .get_one::<String>("source")
        .expect("required source file argument");
    let target = matches
        .get_one::<String>("target")
        .expect("required target file argument");
    let source_ref = matches
        .get_one::<String>("source_ref")
        .expect("required source reference file argument");
    let target_ref = matches
        .get_one::<String>("target_ref")
        .expect("required target referece file argument");

    let verbose = matches.get_one("verbose");

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

    let mut source_ref_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(source_ref)
        .expect("Unable to open file");

    let mut target_ref_file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .truncate(false)
        .create(false) // Optionally create the file if it doesn't already exist
        .open(target_ref)
        .expect("Unable to open file");

    let mut align = Alignment::new();
    align.create_alignemnt(&mut source_file, &mut target_file);

    let mut align_ref = Alignment::new();
    align_ref.create_alignemnt(&mut source_ref_file, &mut target_ref_file);

    match evaluate_alignment(&align, &align_ref) {
        Ok(report) => {
            println!(
                "==== REPORT ====\nTotal alignments: {}\nMatched alignments: {}\nPrecision: {}",
                report.alignments_total,
                report.alignments_matched,
                report.precision()
            );

            let recall: f32 =
                report.alignments_matched as f32 / align_ref.pairs.iter().count() as f32;
            println!("Recall: {}", recall);

            // precision x recall / ((precision + recall) / 2 )
            let f1: f32 = (report.precision() as f32 * recall as f32)
                / ((report.precision() as f32 + recall as f32) / 2_f32);
            println!("F1 score: {}", f1);
            // zaver prace -> contrib->vecaling tool
            // limitations -> manualna kontrola/chyby v referencii
            // recall = report.alignments_matched / celkovy pocet align v ref

            // f1 == harmonic mean of recall and precision

            if let Some(v) = verbose {
                if *v {
                    println!(
                        "Total missalignments: {}",
                        report.alignments_missmatched_source + report.alignments_missmatched_target,
                    );
                    println!(
                        "Possibly missaligned in source: {}",
                        report.alignments_missmatched_source
                    );
                    report
                        .bad_sources
                        .iter()
                        .for_each(|bs| println!("{}:{}", bs.line_number, bs.text));

                    println!(
                        "Possibly missaligned in target: {}",
                        report.alignments_missmatched_target
                    );
                    report
                        .bad_targets
                        .iter()
                        .for_each(|bs| println!("{}:{}", bs.line_number, bs.text));
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
