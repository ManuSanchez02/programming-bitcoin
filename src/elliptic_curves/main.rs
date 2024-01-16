use point::Point;

mod point;
mod coordinate;

fn main() -> Result<(), String> {
    let p1 = Point::new(2, 5, 5.0, 7.0)?;
    let p2 = Point::new(-1, -1, 5.0, 7.0)?;

    let res = (p1 + p2)?;
    println!("{}", res);

    let p1 = Point::new(-1, -1, 5.0, 7.0)?;
    let p2 = Point::new(-1, -1, 5.0, 7.0)?;

    let res = (p1 + p2)?;
    println!("{}", res);

    Ok(())
}
