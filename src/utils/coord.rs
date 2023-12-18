use std::hash::Hash;

use num::{FromPrimitive, PrimInt};

pub trait Coordinate: Hash + Eq + Copy
where
    Self: Sized,
{
    type Value: PrimInt + FromPrimitive;
    type UnsignedLen: PrimInt;
    type SignedLen: PrimInt;

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

pub trait Coordinate2D: Coordinate
where
    Self: Sized,
{
    fn to_tuple(self) -> (Self::Value, Self::Value);

    fn from_tuple(value: (Self::Value, Self::Value)) -> Self;
}
