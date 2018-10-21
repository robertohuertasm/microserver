use console::style;
use warp::Filter;

pub fn start(port: Option<u16>, path: Option<String>, is_spa: bool) {
    let final_port = port.unwrap_or(9090);
    let final_path = path.unwrap_or(".".to_string());
    let spa_index_path = format!("{}/index.html", final_path);

    println!(
        "{}",
        style(format!("MicroServer running on port {}!", final_port))
            .bold()
            .green()
    );
    println!("{}", style(format!("Serving {}", final_path)).bold().blue());
    println!(
        "{}",
        style(format!("Spa support: {}", is_spa)).bold().yellow()
    );

    let files = warp::fs::dir(final_path);
    let spa = warp::any().and(warp::fs::file(spa_index_path));

    let routes = files.or(spa);

    let address = ([0, 0, 0, 0], final_port);
    warp::serve(routes.map(|x| {
        println!("{:?}", x);
        x
    }))
    .run(address);
}
