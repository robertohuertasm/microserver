use console::style;
use warp::Filter;

pub fn start(port: u16, path: String, is_spa: bool, spa_index: &str) {
    let spa_index_path = format!("{}/{}", path, spa_index);
    println!(
        "{}",
        style(format!("MicroServer running on port {}!", port))
            .bold()
            .green()
    );
    println!("{}", style(format!("Serving {}", path)).bold().blue());
    println!(
        "{}",
        style(format!("Spa support: {}. Root: {}", is_spa, spa_index))
            .bold()
            .yellow()
    );

    let files = warp::fs::dir(path);
    let spa = warp::any()
        .and_then(move || {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(spa_index_path))
        .map(|_, file| file);

    let routes = files.or(spa);

    let address = ([0, 0, 0, 0], port);
    warp::serve(routes.map(|file| {
        println!("{:?}", file);
        file
    }))
    .run(address);
}
