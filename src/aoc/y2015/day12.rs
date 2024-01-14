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

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let value = serde_json::from_str(&line).unwrap();
    let ret = recursive_sum(value);
    Ok(ret.to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    crate::multi_line_example(data, main)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "12",
    main,
    main_example
));
