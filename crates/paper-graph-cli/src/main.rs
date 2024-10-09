use clap::{arg, value_parser, Command};
use paper_graph::{generate_paper_graph, StyleOptions};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

fn read_file(path: &PathBuf) -> String {
    let mut buffer = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}

fn write_file(path: &PathBuf, text: &str) {
    File::create(path)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
}

fn main() {
    let matches = Command::new("paper-graph")
        .version("0.1")
        .about("The command line interface for paper-graph.")
        .help_expected(true)
        .arg(
            arg!(-b --bib <FILE> "Path to the .bib file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-g --graph <FILE> "Path to the graph structure file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <FILE> "Path to the output file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--"line-width" <WIDTH> "The maximum line width in nodes")
                .required(false)
                .default_value("32"),
        )
        .get_matches();

    let bib_file = matches.get_one::<PathBuf>("bib").unwrap();
    let graph_file = matches.get_one::<PathBuf>("graph").unwrap();
    let output_file = matches.get_one::<PathBuf>("output").unwrap();

    let options = StyleOptions {
        line_width: matches
            .get_one::<String>("line-width")
            .unwrap()
            .parse()
            .unwrap(),
    };
    let digraph = generate_paper_graph(&read_file(bib_file), &read_file(graph_file), &options);
    write_file(output_file, &digraph);
}
