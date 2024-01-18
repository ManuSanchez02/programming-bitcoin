use coordinate::Coordinate;
use point::Point;

mod coordinate;
mod field_element;
mod is_zero;
mod point;
mod pow;
mod real_value;

fn main() -> Result<(), String> {
    let p1 = Point::from_finite_field(47, 71, 0, 7, 223).unwrap();

    for i in 0..22 {
        let res = i * p1;
        let x = if let Coordinate::Value(x) = res.x {
            x.number
        } else {
            0
        };

        let y = if let Coordinate::Value(y) = res.y {
            y.number
        } else {
            0
        };
        println!("{i}*(47,71) = ({x},{y})");
    }

    Ok(())
}
