use std::hash::Hash;

pub trait Coordinate: Hash + Eq + Copy {
    type UnsignedLen;
    type SignedLen;

    const MAX: Self;
    const MIN: Self;

    fn distance(&self, other: &Self) -> Self::UnsignedLen;

    fn get_max(&self, other: &Self) -> Self;

    fn get_min(&self, other: &Self) -> Self;

    fn is_empty(&self) -> bool;

    fn len(&self) -> f64;

    fn len_sqr(&self) -> Self::SignedLen;

    fn len_manhatten(&self) -> Self::UnsignedLen;
}
