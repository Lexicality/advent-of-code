// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fs::File;
use std::io::prelude::*;

use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header;
use reqwest::StatusCode;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env = "AOC_SESSION_TOKEN")]
    session_token: String,
    #[arg()]
    year: u16,
    #[arg()]
    day: u8,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let year = args.year;
    let day = args.day;
    if year < 2015 {
        return Err(format!("The year {} is too low!", year));
    } else if year > 2038 {
        // just going out on a limb here
        return Err(format!("The year {} is too high!", year));
    } else if day == 0 {
        return Err(format!("The day {} is too low!", day));
    } else if day > 31 {
        return Err(format!("The day {} is too high!", day));
    }

    let client = Client::new();
    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(header::COOKIE, format!("session={}", args.session_token))
        .header(
            header::USER_AGENT,
            "github.com/lexicality/advent-of-code by lexi@lexi.org.uk",
        )
        .send()
        .map_err(|e| e.to_string())?;

    let status = res.status();
    if status != StatusCode::OK {
        return Err(format!(
            "Server responded with status {}\n{}",
            status,
            res.text().map_err(|e| e.to_string())?
        ));
    }

    let data = res.bytes().map_err(|e| e.to_string())?;

    std::fs::create_dir_all(format!("./data/{year}/")).map_err(|e| e.to_string())?;

    let mut f = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(format!("./data/{year}/{day:02}.txt"))
        .map_err(|e| e.to_string())?;

    f.write_all(&data).map_err(|e| e.to_string())?;

    println!("Written to data/{year}/{day:02}.txt!");

    Ok(())
}
