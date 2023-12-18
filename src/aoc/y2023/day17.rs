use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;

use itertools::Itertools;

use crate::{
    utils::direction::RotateDirection, CommonGrid, Coord2D, Coordinate, Direction, FlatGrid, Grid,
};

type NodeID = (Coord2D, Direction, u32);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    id: NodeID,
    neighbours: Vec<NodeID>,
    cost: u32,
    heuristic: u32,
}

impl Node {
    fn new(id: NodeID, neighbours: Vec<NodeID>, cost: u32, heuristic: u32) -> Self {
        Self {
            id,
            neighbours,
            cost,
            heuristic,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct AStarNode<T> {
    f_score: u32,
    id: T,
}

impl<T: Ord> Ord for AStarNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<T: Ord> PartialOrd for AStarNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

enum FinalGridState {
    Untouched(u32),
    Trampled(Direction),
}

impl Display for FinalGridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Untouched(v) => v.fmt(f),
            Self::Trampled(dir) => match dir {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            }
            .fmt(f),
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<u32> = Grid::new_from_lines(
        data.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()),
    );
    println!("{grid:}");
    let start = grid.min_key();
    let end = grid.max_key();

    let capacity_guess = grid.len() * 12;

    let mut seen: HashSet<NodeID> = HashSet::with_capacity(capacity_guess);
    let mut queue: VecDeque<NodeID> = VecDeque::with_capacity(capacity_guess / 2);
    let mut all_nodes: HashMap<NodeID, Node> = HashMap::with_capacity(capacity_guess);

    let start_id = (start, Direction::East, 0);

    queue.push_back(start_id);

    const MAX_STEPPE: u32 = 3;

    // Generate the graph
    while let Some((pos, dir, steps)) = queue.pop_front() {
        let next_steps = [
            (pos + dir.to_coord(), dir, steps + 1),
            {
                let dir = dir.rotate(RotateDirection::Left);
                (pos + dir.to_coord(), dir, 1)
            },
            {
                let dir = dir.rotate(RotateDirection::Right);
                (pos + dir.to_coord(), dir, 1)
            },
        ]
        .into_iter()
        .filter(|(pos, _, steps)| *steps <= MAX_STEPPE && grid.check_coord(pos))
        .collect_vec();
        let node = Node::new(
            (pos, dir, steps),
            next_steps,
            grid.get(&pos).copied().unwrap(),
            pos.distance(&end),
        );
        queue.extend(node.neighbours.iter().filter(|id| seen.insert(**id)));
        all_nodes.insert(node.id, node);
    }
    drop(queue);
    drop(seen);

    // Thanks wikipedia
    let capacity = all_nodes.len();
    let mut came_from: HashMap<NodeID, NodeID> = HashMap::with_capacity(capacity);
    let mut scores: HashMap<NodeID, u32> = HashMap::with_capacity(capacity);
    let mut open_set: BinaryHeap<AStarNode<NodeID>> = BinaryHeap::with_capacity(capacity);

    open_set.push(AStarNode {
        id: start_id,
        f_score: 0,
    });
    scores.insert(start_id, 0);

    let mut grid: Grid<FinalGridState> = grid
        .into_iter()
        .map(|(c, v)| (c, FinalGridState::Untouched(v)))
        .collect();

    while let Some(current) = open_set.pop() {
        if current.id.0 == end {
            let mut ret = 0;
            let mut id = &current.id;
            loop {
                ret += all_nodes.get(id).unwrap().cost;
                grid.set(id.0, FinalGridState::Trampled(id.1));
                id = came_from.get(id).unwrap();
                if id == &start_id {
                    println!("{grid:.>}");
                    return Ok(ret.to_string());
                }
            }
        }
        let current_score = scores.get(&current.id).copied().unwrap();
        for neighbour_id in all_nodes.get(&current.id).unwrap().neighbours.iter() {
            let neighbour = all_nodes.get(neighbour_id).unwrap();
            let maybe_score = current_score + neighbour.cost;
            if scores
                .get(neighbour_id)
                .copied()
                .is_some_and(|score| score <= maybe_score)
            {
                continue;
            }
            scores.insert(*neighbour_id, maybe_score);
            came_from.insert(*neighbour_id, current.id);
            let f_score = maybe_score + neighbour.heuristic;
            open_set.push(AStarNode {
                id: *neighbour_id,
                f_score,
            })
        }
    }

    unreachable!();
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "17",
    func: main,
    example_func: None,
});
