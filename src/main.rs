use clap::{command, Command as Clamp_command};
use libc;
use std::ffi;

#[macro_use]
extern crate lazy_static;

mod settings;
use crate::settings::SETTINGS;

fn main() {
    let matches = command!()
        .subcommands(
            SETTINGS
                .aliases
                .keys()
                .map(|alias| Clamp_command::new(alias))
                .collect::<Vec<_>>(),
        )
        .get_matches();

    match matches.subcommand() {
        Some((external, _)) => {
            let command = SETTINGS.aliases.get(external).unwrap();

            unsafe {
                let command_c = ffi::CString::new(command.clone()).expect("CString::new failed");

                libc::system(command_c.as_ptr());
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

// sui sui [name]
// adds the alias to config, creates config if it doesn't exist

// sui ls
// reads nearest(upwards) configs until a match found, executes it
