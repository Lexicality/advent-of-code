// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

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

    fn try_from_tuple<V: PrimInt>(value: (V, V)) -> Option<Self> {
        Some(Self::from_tuple((num::cast(value.0)?, num::cast(value.1)?)))
    }

    fn get_neighbours(&self) -> [Self; 4];

    fn get_diagonal_neighbours(&self) -> [Self; 8];
}
