// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::time::Instant;

use advent_of_code::{AoCData, AoCDay, AoCResult};
use itertools::Itertools;

type DayMap = HashMap<&'static str, HashMap<&'static str, &'static AoCDay>>;

fn main() -> AoCResult<()> {
    let all_days: DayMap = (inventory::iter::<AoCDay>)
        .into_iter()
        .map(|day_data| (day_data.year, (day_data.day, day_data)))
        .into_group_map()
        .into_iter()
        .map(|(key, days)| (key, days.into_iter().collect()))
        .collect();

    let mut options = clap::command!()
        .arg(clap::arg!(--example "Read the example data file instead"))
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

    let data = AoCData::new_from_file(year, day, use_example)?;

    let func = all_days[year][day].get_function(use_example);

    println!("=== {year} day {day} ===");
    let start = Instant::now();
    let ret = func(data.into_iter())?;
    let end = Instant::now();

    println!(
        "=== Result in {} ===",
        humantime::format_duration(end.duration_since(start))
    );
    println!("{}", ret);
    Ok(())
}
