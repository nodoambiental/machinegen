use super::types::TableTypes;
use super::util;

pub fn run(sub_match: &clap::ArgMatches) {
    match sub_match.subcommand() {
        Some(("table", table_m)) => table(table_m),
        _ => unreachable!(),
    }
}

fn table(table_match: &clap::ArgMatches) {
    match table_match.subcommand() {
        Some(("parse", parse_match)) => table_parse(parse_match),
        Some(("process", parse_match)) => table_process(parse_match),
        _ => unreachable!(),
    }
}

fn table_parse(parse_match: &clap::ArgMatches) {
    // Test table parsing
    println!("{:?}\n", util::load_table(TableTypes::Replace));
    println!("{:?}\n", util::load_table(TableTypes::Files));
    println!("{:?}\n", util::load_table(TableTypes::Template));
}

fn table_process(parse_match: &clap::ArgMatches) {
    // Test table processing

    #[cfg(feature = "debug")]
    util::stdout(
        "debug",
        "\nPrinting the whole machine data relationships struct:",
    );

    println!("{:?}\n", util::process_relations());
}
