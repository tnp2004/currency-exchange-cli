use clap::Parser;
use std::{fmt, process::exit};

enum Country {
    Bath, // Thailand
    Yen, // Japan
    Dollar, // UnitedStates
    Pound, // England
    Euro // French
}

impl fmt::Debug for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bath => write!(f, "bath"),
            Self::Yen => write!(f, "yen"),
            Self::Dollar => write!(f, "dollar"),
            Self::Pound => write!(f, "pound"),
            Self::Euro => write!(f, "euro"),
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
    from_currency: Country,
    to_currency: Country,
    amount: f64
}

impl Currency {
    fn new(from_currency: Country, to_currency: Country, amount: f64) -> Currency {
        Currency{
            from_currency: from_currency,
            to_currency: to_currency,
            amount,
        }
    }

    fn convert_to_central_currency(&self) -> f64 {
        // rate per 1 central rate ( dollar )
        let currency_rate = match self.from_currency {
            Country::Bath => 34.04,
            Country::Yen => 146.21,
            Country::Dollar => 1.0,
            Country::Pound => 0.76,
            Country::Euro => 784.96,
        };
        
        // central currency in this case it's dollar
        self.amount / currency_rate
    }

    fn cal_exchange_currency(&self) -> f64 {
        let central_amount = self.convert_to_central_currency();
        
        let amount = match self.to_currency {
            Country::Bath => 34.04,
            Country::Yen => 146.21,
            Country::Dollar => 1.0,
            Country::Pound => 0.76,
            Country::Euro => 784.96,
        };

        amount * central_amount
    }

    fn cal_and_print_exchange_currency(&self) {
        let amount = self.cal_exchange_currency();
        println!("Exchange from {} to {}, got {:.2}", self.from_currency, self.to_currency, amount)
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
        "bath" => Ok(Country::Bath),
        "yen" => Ok(Country::Yen),
        "dollar" => Ok(Country::Dollar),
        "pound" => Ok(Country::Pound),
        "euro" => Ok(Country::Euro),
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