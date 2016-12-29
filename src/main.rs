#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

extern crate rocket;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use rocket::config::{Config, ConfigError, Environment};
use rocket::response::content::JSON;

static DEFPORT: usize = 32223;
static DEFBIND: &'static str = "127.0.0.1:32223";
static DEFTTCP: &'static str = "127.0.0.1:22222";

lazy_static! {
    static ref M: clap::ArgMatches<'static> = clap_app!(Httcp =>
        (version: "0.1.0")
        (author: "Nurahmadie <nurahmadie@gmail.com>")
        (about: "Proxy TCP server with HTTP frontend")
        (@arg bind: -b --bind +takes_value "Bind to this address")
        (@arg tcp: -t --tcpserver +takes_value "Connect to this server")
    ).get_matches();

    static ref BIND: &'static str = (*M).value_of("bind").unwrap_or(DEFBIND);
    static ref TCPSERVER: &'static str = (*M).value_of("tcp").unwrap_or(DEFTTCP);
}


fn main() {
    run_server(*BIND).unwrap_or_else(|err| err.pretty_print());
}


macro_rules! try_maybe {
	($expr:expr) => (match $expr {
		Ok(val) => val,
		Err(err) => {
			return JSON(format!("{:?}", err));
		}
	})
}

#[get("/nc_stats")]
fn proxify() -> JSON<String> {
    let rconn = TcpStream::connect(*TCPSERVER);
    match rconn {
        Ok(stream) => {
            let mut buff = String::new();
            let mut input = try_maybe!(stream.try_clone());
            try_maybe!(input.set_read_timeout(Some(Duration::new(5, 0))));
            try_maybe!(input.read_to_string(&mut buff));
            JSON(buff)
        }
        Err(err) => {
            JSON(format!("{:?}", err))
        }
    }
}

fn run_server(b: &str) -> Result<(), ConfigError> {
    let bv: Vec<&str> = b.split(":").collect();
    let addr = bv[0].to_owned();
    let port = bv[1].parse::<usize>().unwrap_or(DEFPORT);

    let config = Config::default_for(Environment::active()?, "custom")
        ?
        .address(addr)
        .port(port);

    rocket::custom(&config).mount("/", routes![proxify]).launch();

    Ok(())
}
