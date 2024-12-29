// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::utils::astar;
use crate::{CommonGrid, Coord2D, Coordinate, Direction, FlatGrid, Grid, RotateDirection};

type NodeID = (Coord2D, Direction, u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: NodeID,
    neighbours: Vec<NodeID>,
    cost: u64,
    heuristic: u64,
}

struct Day17Provider {
    all_nodes: HashMap<NodeID, Node>,
    end: Coord2D,
}

impl astar::AStarProvider for Day17Provider {
    type IDType = NodeID;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        Box::new(self.all_nodes.get(id).unwrap().neighbours.iter().copied())
    }

    fn cost(&self, id: &Self::IDType) -> u64 {
        self.all_nodes.get(id).unwrap().cost
    }

    fn heuristic(&self, id: &Self::IDType) -> u64 {
        self.all_nodes.get(id).unwrap().heuristic
    }

    fn is_end(&self, id: &Self::IDType) -> bool {
        id.0 == self.end
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<u32> = Grid::new_from_lines(
        data.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()),
    );
    // println!("{grid:#}");
    let start = grid.min_key();
    let end = grid.max_key();

    let capacity_guess = grid.len() * 12;

    let mut seen: HashSet<NodeID> = HashSet::with_capacity(capacity_guess);
    let mut queue: VecDeque<NodeID> = VecDeque::with_capacity(capacity_guess / 2);
    let mut all_nodes: HashMap<NodeID, Node> = HashMap::with_capacity(capacity_guess);

    let start_id = (start, Direction::East, 0);

    queue.push_back(start_id);

    const MIN_STEPPE: u32 = 4;
    const MAX_STEPPE: u32 = 10;

    // Generate the graph
    while let Some((pos, dir, steps)) = queue.pop_front() {
        let next_steps = [
            {
                if steps < MAX_STEPPE {
                    Some((pos + dir.to_coord(), dir, steps + 1))
                } else {
                    None
                }
            },
            {
                if steps >= MIN_STEPPE {
                    let dir = dir.rotate(RotateDirection::Left);
                    Some((pos + dir.to_coord(), dir, 1))
                } else {
                    None
                }
            },
            {
                if steps >= MIN_STEPPE {
                    let dir = dir.rotate(RotateDirection::Right);
                    Some((pos + dir.to_coord(), dir, 1))
                } else {
                    None
                }
            },
        ]
        .into_iter()
        .flatten()
        .filter(|(pos, _, steps)| grid.check_coord(pos) && (pos != &end || *steps >= MIN_STEPPE))
        .collect_vec();
        let node = Node {
            id: (pos, dir, steps),
            neighbours: next_steps,
            cost: grid.get(&pos).copied().unwrap() as u64,
            heuristic: pos.distance(&end) as u64,
        };
        queue.extend(node.neighbours.iter().filter(|id| seen.insert(**id)));
        all_nodes.insert(node.id, node);
    }
    drop(queue);
    drop(seen);

    let provider = Day17Provider { all_nodes, end };

    let res = astar::a_star(provider, start_id);

    Ok(res
        .into_iter()
        .map(|id| grid.get(&id.0).unwrap())
        .sum::<u32>()
        .to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "17",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
