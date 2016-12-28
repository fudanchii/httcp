#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;

extern crate rocket;

use rocket::config::{Config, Environment};

static DEFPORT: usize = 32223;
static DEFBIND: &'static str = "127.0.0.1:32223";
static DEFTTCP: &'static str = "127.0.0.1:22222";

#[get("/")]
fn proxify() -> &'static str {
    "test\n"
}

fn run_server(b: &str, t: &str) -> Result<(), HttcpError> {
    let bv: Vec<&str> = b.split(":").collect();
    let addr = bv[0].to_owned();
    let port = bv[1].parse::<usize>().unwrap_or(DEFPORT);

    let config = Config::default_for(Environment::active()?, "custom")?
                       .address(addr)
                       .port(port);

    rocket::custom(&config).mount("/", routes![proxify]).launch();

    Ok(())
}

fn main() {
    let matches = clap_app!(app =>
        (version: "0.1.0")
        (author: "Nurahmadie <nurahmadie@gmail.com>")
        (about: "Proxy TCP server with http frontend")
        (@arg bind: -b --bind +takes_value "Bind to this address")
        (@arg tcp: -t --tcpserver +takes_value "Connect to this server")
    ).get_matches();

    let bind = matches.value_of("bind").unwrap_or(DEFBIND);
    let tcpserver = matches.value_of("tcp").unwrap_or(DEFTTCP);

    run_server(bind, tcpserver).or_else(|err| println!("{}", err.description()));
}