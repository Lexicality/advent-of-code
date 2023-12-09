use advent_of_code::{AoCDay, AoCDayFn};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::PathBuf;

struct DayFunctions {
    func: AoCDayFn,
    example_func: Option<AoCDayFn>,
}

type DayMap = BTreeMap<&'static str, BTreeMap<&'static str, DayFunctions>>;

fn main() {
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
        eprintln!(
            "Cannot read data file for {}/{}: {} does not exist!",
            year,
            day,
            data_path.to_string_lossy()
        );
        return;
    }
    let mut lines = BufReader::new(File::open(data_path).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let funcs = all_days.get(year).unwrap().get(day).unwrap();

    let func = if use_example {
        funcs.example_func.unwrap_or(funcs.func)
    } else {
        funcs.func
    };

    let ret = func(&mut lines);

    println!("=== Result ===");
    println!("{}", ret);
}
