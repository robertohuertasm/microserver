mod server;

use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A micro server to run from your CLI with support for SPAs.\nBased on Warp!")
        .author("Roberto Huertas <roberto.huertas@outlook.com>")
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("9090")
                .takes_value(true)
                .help("Sets the port."),
        )
        .arg(
            Arg::new("address")
                .long("address")
                .short('a')
                .default_value("0.0.0.0")
                .help("Sets the address to use."),
        )
        .arg(
            Arg::new("no-spa")
                .long("no-spa")
                .short('n')
                .help("Removes support for Single Page Applications"),
        )
        .arg(
            Arg::new("spa-index")
                .long("spa-index")
                .short('i')
                .default_value("index.html")
                .takes_value(true)
                .help("Sets the name of the index document."),
        )
        .arg(
            Arg::new("path")
                .default_value(".")
                .help("The path to the files being served")
                .index(1),
        )
        .get_matches();

    let port = matches
        .value_of("port")
        .and_then(|x| x.parse::<u16>().ok())
        .unwrap();
    let path = matches.value_of("path").unwrap();
    let is_spa = !matches.is_present("no-spa");
    let spa_index = matches.value_of("spa-index").unwrap();
    let address = matches.value_of("address").unwrap();

    // TODO: verbose & param validations!
    server::start(port, path.to_owned(), is_spa, spa_index, address).await;
}
