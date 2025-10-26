// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    filename: PathBuf,
}

struct Node {
    input_1: String,
    input_2: String,
    output: String,
    label: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = Args::parse().filename;
    if !filename.is_file() {
        return Err("Must be a data file!".into());
    }

    let lines: Vec<_> = BufReader::new(File::open(filename)?)
        .lines()
        .try_collect()?;

    let gate_re = Regex::new(r"^(.+) (.+) (.+) -> (.+)$")?;

    let nodes: Vec<Node> = lines
        .into_iter()
        .filter_map(|line| {
            let matches = gate_re.captures(&line)?;
            Some(Node {
                input_1: matches[1].to_owned(),
                label: matches[2].to_owned(),
                input_2: matches[3].to_owned(),
                output: matches[4].to_owned(),
            })
        })
        .sorted_by_cached_key(|n| n.output.clone())
        .collect();

    let input_map: HashMap<&str, Vec<&str>> = nodes.iter().fold(HashMap::new(), |mut acc, node| {
        acc.entry(&node.input_1).or_default().push(&node.output);
        acc.entry(&node.input_2).or_default().push(&node.output);
        acc
    });

    println!("strict digraph {{");

    let x_inputs: Vec<_> = input_map
        .keys()
        .filter(|k| k.starts_with("x"))
        .sorted()
        .collect();
    let y_inputs: Vec<_> = input_map
        .keys()
        .filter(|k| k.starts_with("y"))
        .sorted()
        .collect();

    println!(
        "{{ edge[style=invis]; rank=same {} }}",
        x_inputs
            .iter()
            .zip(y_inputs.iter())
            .flat_map(|(a, b)| [a, b])
            .join("->")
    );

    for input in x_inputs {
        println!("\t\t\t{input}; {input} [label=\"{input}\",style=filled,fillcolor=\"#7CA982\"];");
    }
    for input in y_inputs {
        println!("\t\t\t{input}; {input} [label=\"{input}\",style=filled,fillcolor=\"#C2A83E\"];");
    }

    for node in nodes.iter() {
        println!(
            "\t\t{} [label=\"{}\",style=filled,fillcolor=\"{}\"];",
            node.output,
            node.label,
            {
                match node.label.as_str() {
                    "AND" => "#084C61",
                    "XOR" => "#DB3A34",
                    "OR" => "#FFC857",
                    _ => "blue",
                }
            }
        );
    }
    // println!("\t}}");
    for (input, target) in input_map
        .into_iter()
        .flat_map(|(input, targets)| targets.into_iter().map(move |target| (input, target)))
        .sorted()
    {
        println!(
            "\t{} -> {} [label=\"{}\", style=\"{}\"];",
            input,
            target,
            input,
            {
                if input.starts_with('x') {
                    "dotted"
                } else if input.starts_with('y') {
                    "dashed"
                } else {
                    "solid"
                }
            }
        );
    }

    for output in nodes
        .into_iter()
        .map(|node| node.output)
        .filter(|output| output.starts_with("z"))
        .sorted()
    {
        println!(
            "\t{output} -> Z [taillabel=\"{output}\",label=\"{output}\", color=\"#243E36\", style=bold];",
        );
    }

    println!("edge [labelfloat=true];");

    println!("}}");
    Ok(())
}
