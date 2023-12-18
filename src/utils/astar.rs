use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AStarNode<IDType>
where
    IDType: Ord,
{
    f_score: u64,
    id: IDType,
}

impl<ID: Ord> Ord for AStarNode<ID> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<ID: Ord> PartialOrd for AStarNode<ID> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub trait AStarProvider {
    type IDType: Ord + Sized;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_>;
    fn heuristic(&self, id: &Self::IDType) -> u64;
    fn cost(&self, id: &Self::IDType) -> u64;
    fn is_end(&self, id: &Self::IDType) -> bool;
}

pub fn a_star<IDType, Provider>(provider: Provider, start_id: IDType) -> Vec<IDType>
where
    IDType: Ord + Copy + Hash,
    Provider: AStarProvider<IDType = IDType>,
{
    // Thanks wikipedia
    let mut came_from: HashMap<IDType, IDType> = HashMap::new();
    let mut scores: HashMap<IDType, u64> = HashMap::new();
    let mut open_set: BinaryHeap<AStarNode<IDType>> = BinaryHeap::new();

    open_set.push(AStarNode {
        id: start_id,
        f_score: 0,
    });
    scores.insert(start_id, 0);

    while let Some(AStarNode { id, f_score: _ }) = open_set.pop() {
        if provider.is_end(&id) {
            let mut ret = vec![];
            let mut id = &id;
            loop {
                ret.push(*id);
                id = came_from.get(id).unwrap();
                if id == &start_id {
                    return ret;
                }
            }
        }
        let current_score = scores.get(&id).copied().unwrap();
        for neighbour_id in provider.get_neighbours(&id) {
            let maybe_score = current_score + provider.cost(&neighbour_id);
            if scores
                .get(&neighbour_id)
                .copied()
                .is_some_and(|score| score <= maybe_score)
            {
                continue;
            }
            scores.insert(neighbour_id, maybe_score);
            came_from.insert(neighbour_id, id);
            let f_score = maybe_score + provider.heuristic(&neighbour_id);
            open_set.push(AStarNode {
                id: neighbour_id,
                f_score,
            })
        }
    }

    eprintln!("Unable to route from start to end!");

    vec![]
}
