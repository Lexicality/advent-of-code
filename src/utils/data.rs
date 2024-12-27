/**
 * Copyright (c) 2024 Lexi Robinson
 *
 * Licensed under the EUPL, Version 1.2
 *
 * You may not use this work except in compliance with the Licence.
 * You should have received a copy of the Licence along with this work. If not, see:
 * <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
 * See the Licence for the specific language governing permissions and limitations under the Licence.
 */
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FusedIterator;
use std::path::PathBuf;

use itertools::Itertools;

use crate::{AoCError, AoCResult, InputPartitioner};

type Line = String;
type Lines = Vec<Line>;
type LinesIter = <Lines as IntoIterator>::IntoIter;

pub struct AoCDataIterator(LinesIter);

impl Iterator for AoCDataIterator {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl FusedIterator for AoCDataIterator {}
impl ExactSizeIterator for AoCDataIterator {}
impl DoubleEndedIterator for AoCDataIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl AoCDataIterator {
    pub fn partition(self) -> impl Iterator<Item = AoCData> {
        InputPartitioner::new(self, |line| !line.is_empty()).map(|data| data.into())
    }
}

#[derive(Clone, Debug, Default)]
pub struct AoCData(Lines);

impl IntoIterator for AoCData {
    type Item = Line;

    type IntoIter = AoCDataIterator;

    fn into_iter(self) -> Self::IntoIter {
        AoCDataIterator(self.0.into_iter())
    }
}

impl AoCData {
    pub fn new_from_line(line: String) -> Self {
        vec![line].into()
    }

    pub fn new_from_file(year: &str, day: &str, example: bool) -> AoCResult<Self> {
        let mut data_path: PathBuf = [".", "data", year].iter().collect();
        if example {
            data_path.push("example");
        }
        data_path.push(format!("{:0>2}", day));
        data_path.set_extension("txt");
        let data_path_str = data_path.to_string_lossy().to_string();
        Ok(Self(
            BufReader::new(File::open(data_path).map_err(|cause| {
                AoCError::new_with_cause(format!("Failed to open {data_path_str}"), cause)
            })?)
            .lines()
            .try_collect()
            .map_err(|cause| {
                AoCError::new_with_cause(format!("Failed to read line from {data_path_str}"), cause)
            })?,
        ))
    }
}

impl From<Vec<String>> for AoCData {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl FromIterator<String> for AoCData {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
