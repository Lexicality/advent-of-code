// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::iter::{FusedIterator, Peekable};

use itertools::Itertools;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct InputPartitioner<Iter: Iterator> {
    inner: Peekable<Iter>,
    pred: fn(&Iter::Item) -> bool,
}

impl<Iter: Iterator> InputPartitioner<Iter> {
    pub fn new<Input>(iter: Input, pred: fn(&Iter::Item) -> bool) -> Self
    where
        Input: IntoIterator<IntoIter = Iter>,
    {
        Self {
            inner: iter.into_iter().peekable(),
            pred,
        }
    }
}

impl<Iter: Iterator> Iterator for InputPartitioner<Iter> {
    type Item = Vec<Iter::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.peek()?;
        Some(self.inner.by_ref().take_while(self.pred).collect_vec())
    }
}

impl<Iter: Iterator<Item = String>> FusedIterator for InputPartitioner<Iter> {}
