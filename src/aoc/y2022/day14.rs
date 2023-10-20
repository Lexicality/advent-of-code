use crate::Coord2D;

pub fn main(data: crate::DataIn) -> String {
    for line in data {
        for coord in line.split(" -> ") {
            let coord = Coord2D::parse(coord);
            println!("{}", coord);
        }
    }
    String::new()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "14",
    func: main,
});
