mod cpu;
mod instructions;
mod rom;
mod commands;
mod bus;

extern crate clap;
use clap::{App, Arg, SubCommand};

use crate::commands::{Info, Command, Log};

fn main() {
    let app = App::new("NES Play")
        .version("0.0.1")
        .about("Yet another NES emulator")
        .subcommand(SubCommand::with_name("info")
            .about("Show ROM info")
            .arg(Arg::with_name("ROM").required(true))
        )
        .subcommand(SubCommand::with_name("log")
            .about("Generate execution log for ROM")
            .arg(Arg::with_name("ROM").required(true))
            .arg(Arg::with_name("LOG").required(true))
        );

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        let filename = matches.value_of("ROM").unwrap();
        Info::new(filename).execute();
    }

    if let Some(matches) = matches.subcommand_matches("log") {
        let rom_filename = matches.value_of("ROM").unwrap();
        let log_filename = matches.value_of("LOG").unwrap();

        let command = Log::new(rom_filename, log_filename);
        command.execute();
    }
}
