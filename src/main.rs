#![allow(dead_code)]
#![allow(unused_variables)]

mod server;

use clap::{App, Arg};

fn main() {
    let matches = App::new("microserver")
        .version("0.0.1")
        .about("A micro server to run from your CLI with support for SPAs.\nBased on Warp!")
        .author("Roberto Huertas <roberto.huertas@outlook.com>")
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .help("Sets the port. Defaults to 9090"),
        )
        .arg(
            Arg::with_name("no-spa")
                .long("no-spa")
                .short("ns")
                .help("Removes support for Single Page Applications"),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("t")
                .help("The path to the files being served")
                .index(1),
        )
        .get_matches();

    let port = matches.value_of("port").and_then(|x| x.parse::<u16>().ok());
    let path = matches.value_of("path").and_then(|x| Some(x.to_string()));
    let is_spa = !matches.is_present("no-spa");

    // TODO: verbose & param validations!

    server::start(port, path, is_spa);
}
