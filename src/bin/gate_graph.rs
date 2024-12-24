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
use std::io::prelude::*;
use std::io::BufReader;
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
    #[allow(clippy::single_element_loop)]
    for (key, value) in [
        // global settings
        ("rankdir", "LR"),
        ("ordering", "in"),
        // ("newrank", "true"),
    ] {
        println!("\t{key}=\"{value}\";");
    }
    println!("\tsubgraph cluster_input {{ X; Y;");
    println!("\t\tgraph [label=\"Input\"];");

    println!("\t\tsubgraph cluster_x {{");
    println!("\t\t\tgraph [label=\"X\"]; ");

    let x_inputs: Vec<_> = input_map
        .keys()
        .filter(|k| k.starts_with("x"))
        .sorted()
        .collect();

    // print!(
    //     "\t\t\t{{ edge [style=invis]; {}; }}",
    //     x_inputs.iter().join(" -> ")
    // );

    for input in x_inputs.into_iter() {
        println!("\t\t\tX -> {input}; {input} [label=\"{input}\"];");
    }

    println!("\t\t}}");

    println!("\t\tsubgraph cluster_y {{");

    println!("\t\t\tgraph [label=\"Y\", ordering=in];");
    let y_inputs: Vec<_> = input_map
        .keys()
        .filter(|k| k.starts_with("y"))
        .sorted()
        .collect();

    // print!(
    //     "\t\t\t{{ edge [style=invis]; {}; }}",
    //     y_inputs.iter().join(" -> ")
    // );

    for input in y_inputs {
        println!("\t\t\tY -> {input}; {input} [label=\"{input}\"];");
    }
    println!("\t\t}}");
    // println!("\t\t{{ rank=same; cluster_x; cluster_y; }}");
    println!("\t}}");
    println!("\tsubgraph cluster_output {{ graph [label=\"Output\"]; Z; }}");
    println!("\tsubgraph cluster_gates {{ graph [label=\"Gates\"];");
    for node in nodes.iter() {
        println!("\t\t{} [label=\"{}\"];", node.output, node.label);
    }
    println!("\t}}");
    for (input, target) in input_map
        .into_iter()
        .flat_map(|(input, targets)| targets.into_iter().map(move |target| (input, target)))
        .sorted()
    {
        println!("\t{} -> {} [label=\"{}\"];", input, target, input);
    }

    for output in nodes
        .into_iter()
        .map(|node| node.output)
        .filter(|output| output.starts_with("z"))
        .sorted()
    {
        println!("\t{} -> Z [label=\"{}\"];", output, output);
    }

    println!("}}");
    Ok(())
}
