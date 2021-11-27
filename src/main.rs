mod cpu;
mod instructions;
mod rom;
mod commands;
mod bus;

extern crate clap;
use clap::{App, Arg, SubCommand};

use crate::commands::{Info, Command, Assembly};

fn main() {
    let app = App::new("NES Play")
        .version("0.0.1")
        .about("Yet another NES emulator")
        .subcommand(SubCommand::with_name("info")
            .about("Show ROM info")
            .arg(Arg::with_name("ROM").required(true))
        )
        .subcommand(SubCommand::with_name("assembly")
            .about("Print ASM code for ROM")
            .arg(Arg::with_name("ROM").required(true))
        );

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        let filename = matches.value_of("ROM").unwrap();
        Info::new(filename).execute();
    }

    if let Some(matches) = matches.subcommand_matches("assembly") {
        let filename = matches.value_of("ROM").unwrap();
        Assembly::new(filename).execute();
    }
}
