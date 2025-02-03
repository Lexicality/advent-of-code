// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::Coord2D;

fn power_level(coord: Coord2D, serial: i64) -> i32 {
    let x: i64 = coord.x.into();
    let y: i64 = coord.y.into();
    let rack_id = x + 10;
    let power_level = ((rack_id * y) + serial) * rack_id;
    let power_level = ((power_level / 100) % 10) as i32;
    power_level - 5
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let serial = data.next().unwrap().parse()?;

    let to_check = 1..=(300 - 3);
    let to_glom = 0_i32..3;

    let ret = to_check
        .clone()
        .cartesian_product(to_check)
        .map(|coord| coord.into())
        .sorted_by_cached_key(|coord: &Coord2D| {
            to_glom
                .clone()
                .cartesian_product(to_glom.clone())
                .map(|coord| coord.into())
                .map(|nudge| power_level(coord + &nudge, serial))
                .sum::<i32>()
        })
        .next_back()
        .unwrap();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| crate::multi_line_example(data, part_1),
    },
    part_2: None
});

#[cfg(test)]
mod test {
    use crate::Coord2D;

    use super::power_level;

    #[test]
    fn test_example_1() {
        assert_eq!(power_level(Coord2D { x: 3, y: 5 }, 8), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(power_level(Coord2D { x: 122, y: 79 }, 57), -5);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(power_level(Coord2D { x: 217, y: 196 }, 39), 0);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(power_level(Coord2D { x: 101, y: 153 }, 71), 4);
    }
}
