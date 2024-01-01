use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use advent_of_code::{AoCDay, AoCDayFn, AoCError, AoCResult};
use itertools::Itertools;

struct DayFunctions {
    func: AoCDayFn,
    example_func: Option<AoCDayFn>,
}

type DayMap = BTreeMap<&'static str, BTreeMap<&'static str, DayFunctions>>;

fn main() -> AoCResult<()> {
    let mut all_days: DayMap = BTreeMap::new();

    for AoCDay {
        day,
        year,
        func,
        example_func,
    } in inventory::iter::<AoCDay>
    {
        if !all_days.contains_key(year) {
            all_days.insert(year, BTreeMap::new());
        }

        let year_data = all_days.get_mut(year).unwrap();
        year_data.insert(
            day,
            DayFunctions {
                func: *func,
                example_func: *example_func,
            },
        );
    }

    let mut options = clap::command!()
        .arg(clap::arg!(--example "Read the example data file instead"))
        .subcommand_required(true);
    for (year, days) in all_days.iter() {
        options = options.subcommand(
            clap::Command::new(year)
                .arg(clap::Arg::new("day").value_parser(days.keys().cloned().collect::<Vec<_>>())),
        );
    }
    let matches = options.get_matches();
    let (year, year_args) = matches.subcommand().unwrap();
    let day = &year_args.get_one::<String>("day").unwrap()[..];
    let use_example = matches.get_flag("example");

    let mut data_path: PathBuf = [".", "data", year].iter().collect();
    if use_example {
        data_path.push("example");
    }
    data_path.push(format!("{:0>2}", day));
    data_path.set_extension("txt");
    if !data_path.exists() {
        return Err(AoCError::new(format!(
            "Cannot read data file for {year}/{day}: {} does not exist!",
            data_path.to_string_lossy()
        )));
    }
    let lines = BufReader::new(File::open(data_path).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let funcs = all_days.get(year).unwrap().get(day).unwrap();

    let func = if use_example {
        funcs.example_func.unwrap_or(funcs.func)
    } else {
        funcs.func
    };

    println!("=== {year} day {day} ===");
    let start = Instant::now();
    let ret = func(lines.collect_vec().into_iter())?;
    let end = Instant::now();

    println!(
        "=== Result in {} ===",
        humantime::format_duration(end.duration_since(start))
    );
    println!("{}", ret);
    Ok(())
}
