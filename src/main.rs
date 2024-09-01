use clap::Parser;
use std::{fmt, process::exit};

enum Country {
    Thailand,
    Japan,
    UnitedStates,
    England,
    French
}

impl fmt::Debug for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thailand => write!(f, "Thailand"),
            Self::Japan => write!(f, "Japan"),
            Self::UnitedStates => write!(f, "UnitedStates"),
            Self::England => write!(f, "England"),
            Self::French => write!(f, "French"),
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser)]
struct CliArgs {
    from: String,
    to: String,
    amount: u64
}

fn main() {
    let args = CliArgs::parse();
    let from = match_country(args.from);
    let to = match_country(args.to);
    println!("from {:?} to {}", from, to)
}

fn match_country(arg: String) -> Country {
    let country = match arg.trim().to_lowercase().as_str() {
        "thailand" => Ok(Country::Thailand),
        "japan" => Ok(Country::Japan),
        "unitedstates" => Ok(Country::UnitedStates),
        "england" => Ok(Country::England),
        "french" => Ok(Country::French),
        _ => Err(format!("{} is not available", arg).to_string())
    };

    match country {
        Ok(from) => from,
        Err(e) => {
            println!("{}", e);
            exit(0)
        },
    }
}