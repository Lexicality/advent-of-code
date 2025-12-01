// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::process::ExitCode;
use std::time::Instant;

use env_logger::Builder;
use itertools::Itertools;
use log::LevelFilter;

use advent_of_code::{AoCData, AoCDay, AoCError, AoCResult};

type DayMap = HashMap<&'static str, HashMap<&'static str, &'static AoCDay>>;

fn main_wrapped() -> AoCResult<()> {
    let all_days: DayMap = (inventory::iter::<AoCDay>)
        .into_iter()
        .map(|day_data| (day_data.year, (day_data.day, day_data)))
        .into_group_map()
        .into_iter()
        .map(|(key, days)| (key, days.into_iter().collect()))
        .collect();

    let mut options = clap::command!()
        .arg(clap::arg!(--example "Read the example data file instead"))
        .arg(
            clap::Arg::new("part")
                .long("part")
                .default_value("both")
                .value_parser(["1", "2", "both"]),
        )
        .subcommand_required(true);
    for (year, days) in all_days.iter() {
        options = options.subcommand(
            clap::Command::new(year).arg(
                clap::Arg::new("day")
                    .required(true)
                    .value_parser(days.keys().collect_vec()),
            ),
        );
    }
    let matches = options.get_matches();
    let (year, year_args) = matches.subcommand().unwrap();
    let day = year_args.get_one::<String>("day").unwrap().as_str();
    let use_example = matches.get_flag("example");
    let part = matches.get_one::<String>("part").unwrap().as_str();

    let data = AoCData::new_from_file(year, day, use_example)?;

    let day_data = all_days[year][day];

    // sanity checking
    if part == "2" && day_data.part_2.is_none() {
        return Err(AoCError::new(format!(
            "{year}/{day} does not have a part 2 defined!"
        )));
    }

    if part == "1" || part == "both" {
        println!("=== {year} day {day} part 1 ===");

        let func = {
            let part = &day_data.part_1;
            if use_example { part.example } else { part.main }
        };

        let start = Instant::now();
        let ret = func(data.clone().into_iter())?;
        let end = Instant::now();

        println!(
            "=== Result in {} ===",
            humantime::format_duration(end.duration_since(start))
        );
        println!("{}", ret);
    }
    if part == "2" || (part == "both" && day_data.part_2.is_some()) {
        println!("=== {year} day {day} part 2 ===");

        let func = day_data
            .part_2
            .as_ref()
            .map(|part| if use_example { part.example } else { part.main })
            .unwrap();

        let start = Instant::now();
        let ret = func(data.into_iter())?;
        let end = Instant::now();

        println!(
            "=== Result in {} ===",
            humantime::format_duration(end.duration_since(start))
        );
        println!("{}", ret);
    }
    Ok(())
}

fn main() -> ExitCode {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format_target(false)
        .format_timestamp(None)
        .parse_default_env()
        .init();
    match main_wrapped() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
