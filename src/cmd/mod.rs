use crate::app;
use clap;
use redis;

pub fn boost() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("http-server-app")
        .about("https server app")
        .arg(
            clap::Arg::with_name("redis")
                .short("r")
                .long("redis")
                .help("redis address")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("bind")
                .short("b")
                .long("bind")
                .help("bind address")
                .takes_value(true),
        )
        .get_matches();
    let redis_addr = matches.value_of("redis").unwrap().to_string();
    let bind = matches.value_of("bind").unwrap().to_string();
    let client = redis::Client::open(redis_addr)?;
    app::run(bind, client)?;
    Ok(())
}
