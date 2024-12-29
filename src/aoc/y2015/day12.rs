// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use serde_json::Value as JSONValue;

fn recursive_sum(value: JSONValue) -> i64 {
    match value {
        JSONValue::Bool(_) | JSONValue::Null | JSONValue::String(_) => 0,
        JSONValue::Number(i) => i.as_i64().unwrap(),
        JSONValue::Array(arr) => arr.into_iter().map(recursive_sum).sum(),
        JSONValue::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                0
            } else {
                obj.into_iter().map(|(_, v)| v).map(recursive_sum).sum()
            }
        }
    }
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let value = serde_json::from_str(&line).unwrap();
    let ret = recursive_sum(value);
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "12",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
