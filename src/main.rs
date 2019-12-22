use std::io;
use std::fs;
use std::collections::HashMap;

use mininip::parse;
use mininip::errors::ParseFileError;
use mininip::datas::{Identifier, Value};

fn main() -> Result<(), io::Error> {
    loop {
        let path = loop {
            println!("Which file do want you to parse?");

            let mut path = String::new();
            if io::stdin().read_line(&mut path)? == 0 {
                println!("Good bye!");
                return Ok(());
            }

            let path = String::from(path.trim());

            let metadata = match fs::metadata(&path) {
                Ok(val)  => val,
                Err(err) => {
                    eprintln!("Could not open {}: {}", path, err);
                    continue;
                },
            };

            if metadata.is_file() {
                break path;
            }

            eprintln!("{} is not a file", path);
        };

        match parse::parse_file(&path) {
            Ok(data)                             => explore(data),
            Err(ParseFileError::IOError(err))    => eprintln!("Unable to read this file: {}", err),
            Err(ParseFileError::ParseError(err)) => eprintln!("Syntaxe error in this file: {}", err),
        }
    }
}

fn explore(data: HashMap<Identifier, Value>) {
    loop {
        println!("Enter a section name");
        let mut section = String::new();
        if io::stdin().read_line(&mut section).unwrap() == 0 {
            return;
        }

        let section = section.trim();
        let section = if section.is_empty() {
            None
        } else {
            if !Identifier::is_valid(section) {
                eprintln!("{} is not a valid section name", section);
                continue;
            }
            Some(String::from(section))
        };

        println!("Enter a variable name");
        let mut variable = String::new();
        if io::stdin().read_line(&mut variable).unwrap() == 0 {
            return;
        }

        let variable = variable.trim();
        if !Identifier::is_valid(variable) {
            eprintln!("{} is not a valid variable name", variable);
            continue;
        }

        let identifier = Identifier::new(section, String::from(variable));
        match data.get(&identifier) {
            Some(value) => println!("{}", value),
            None        => eprintln!("Variable not found!"),
        }
    }
}
