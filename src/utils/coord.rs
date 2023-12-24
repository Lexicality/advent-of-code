use std::hash::Hash;

use num::{FromPrimitive, PrimInt};

pub trait Coordinate: Hash + Eq + Copy
where
    Self: Sized,
{
    type Type: PrimInt + FromPrimitive;

    const MAX: Self;
    const MIN: Self;

    fn distance(&self, other: &Self) -> Self::Type;

    fn get_max(&self, other: &Self) -> Self;

    fn get_min(&self, other: &Self) -> Self;

    fn is_empty(&self) -> bool;

    fn len(&self) -> f64;

    fn len_sqr(&self) -> Self::Type;

    fn len_manhatten(&self) -> Self::Type;
}

pub trait Coordinate2D: Coordinate
where
    Self: Sized,
{
    fn to_tuple(self) -> (Self::Type, Self::Type);

    fn from_tuple(value: (Self::Type, Self::Type)) -> Self;
}
