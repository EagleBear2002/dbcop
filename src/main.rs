extern crate chrono;
extern crate clap;
extern crate dbcop;
extern crate rayon;
extern crate serde_yaml;

// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use std::path::Path;

use std::fs;

use dbcop::db::history::generate_mult_histories;
use dbcop::db::history::History;
use dbcop::verifier::Verifier;

fn main() {
    let app = App::new("dbcop")
        .version("1.0")
        .author("Ranadeep")
        .about("Generates histories or verifies executed histories")
        .subcommands(vec![
            SubCommand::with_name("generate")
                .arg(
                    Arg::with_name("g_directory")
                        .long("gen_dir")
                        .short("d")
                        .takes_value(true)
                        .required(true)
                        .help("Directory to generate histories"),
                )
                .arg(
                    Arg::with_name("n_history")
                        .long("nhist")
                        .short("h")
                        .default_value("10")
                        .help("Number of histories to generate"),
                )
                .arg(
                    Arg::with_name("n_node")
                        .long("nnode")
                        .short("n")
                        .default_value("3")
                        .help("Number of nodes per history"),
                )
                .arg(
                    Arg::with_name("n_variable")
                        .long("nval")
                        .short("v")
                        .default_value("5")
                        .help("Number of variables per history"),
                )
                .arg(
                    Arg::with_name("n_transaction")
                        .long("ntxn")
                        .short("t")
                        .default_value("5")
                        .help("Number of transactions per history"),
                )
                .arg(
                    Arg::with_name("n_event")
                        .long("nevt")
                        .short("e")
                        .default_value("2")
                        .help("Number of events per transactions"),
                )
                .about("Generate histories"),
            SubCommand::with_name("verify")
                .arg(
                    Arg::with_name("v_directory")
                        .long("ver_dir")
                        .short("d")
                        .takes_value(true)
                        .required(true)
                        .help("Directory containing executed histories"),
                )
                .arg(
                    Arg::with_name("o_directory")
                        .long("out_dir")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("Directory to output the results."),
                )
                .about("Verifies histories"),
        ])
        .setting(AppSettings::SubcommandRequired);

    let app_matches = app.get_matches();

    if let Some(matches) = app_matches.subcommand_matches("generate") {
        let dir = Path::new(matches.value_of("g_directory").unwrap());

        if !dir.is_dir() {}

        let mut histories = generate_mult_histories(
            matches.value_of("n_history").unwrap().parse().unwrap(),
            matches.value_of("n_node").unwrap().parse().unwrap(),
            matches.value_of("n_variable").unwrap().parse().unwrap(),
            matches.value_of("n_transaction").unwrap().parse().unwrap(),
            matches.value_of("n_event").unwrap().parse().unwrap(),
        );

        for hist in histories.drain(..) {
            let mut file =
                File::create(dir.join(format!("hist-{:05}.json", hist.get_id()))).unwrap();
            let mut buf_writer = BufWriter::new(file);
            serde_json::to_writer_pretty(buf_writer, &hist)
                .expect("dumping history to json file went wrong");
        }
    } else if let Some(matches) = app_matches.subcommand_matches("verify") {
        let v_dir = Path::new(matches.value_of("v_directory").unwrap());

        if !v_dir.is_dir() {}

        let histories: Vec<History> = fs::read_dir(v_dir)
            .unwrap()
            .filter_map(|entry_res| match entry_res {
                Ok(ref entry) if entry.path().is_dir() => {
                    let file = File::open(entry.path().join("history.json")).unwrap();
                    let buf_reader = BufReader::new(file);
                    Some(serde_json::from_reader(buf_reader).unwrap())
                }
                _ => None,
            })
            .collect();

        // println!("{:?}", histories);

        let o_dir = Path::new(matches.value_of("o_directory").unwrap());

        histories.iter().for_each(|ref hist| {
            let curr_dir = o_dir.join(format!("hist-{:05}", hist.get_id()));

            let verifier = Verifier::new(curr_dir.to_path_buf());

            verifier.transactional_history_verify(hist.get_data());
        });
    }
}
