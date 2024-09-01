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
    amount: f64
}

struct Currency {
    central_currency: Country,
    from_currency: Country,
    to_currency: Country,
    amount: f64
}

impl Currency {
    fn new(from_currency: Country, to_currency: Country, amount: f64) -> Currency {
        Currency{
            central_currency: Country::UnitedStates,
            from_currency: from_currency,
            to_currency: to_currency,
            amount,
        }
    }

    fn convert_to_central_currency(&self) -> f64 {
        let currency_rate = match self.from_currency {
            Country::Thailand => 34.04,
            Country::Japan => 146.21,
            Country::UnitedStates => 1.0,
            Country::England => 0.76,
            Country::French => 784.96,
        };
        
        // central currency in this case it's dollar
        self.amount / currency_rate
    }

    fn cal_exchange_currency(&self) -> f64 {
        let central_amount = self.convert_to_central_currency();
        
        let amount = match self.to_currency {
            Country::Thailand => 34.04,
            Country::Japan => 146.21,
            Country::UnitedStates => 1.0,
            Country::England => 0.76,
            Country::French => 784.96,
        };

        amount * central_amount
    }

    fn cal_and_print_exchange_currency(&self) {
        let amount = self.cal_exchange_currency();
        println!("Exchange from {} to {}, got {}", self.from_currency, self.to_currency, amount)
    }
}

fn main() {
    let args = CliArgs::parse();
    let from = match_country(args.from);
    let to = match_country(args.to);
    let currency = Currency::new(from, to, args.amount);
    currency.cal_and_print_exchange_currency();
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