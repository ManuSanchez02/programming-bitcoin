use field_element::FieldElement;

mod field_element;
mod point;

fn main() -> Result<(), String> {
    let prime = 31;
    let set_res: Result<Vec<FieldElement>, String> =
        (0..prime).map(|x| FieldElement::new(x, prime)).collect();
    let set = set_res.unwrap();
    let set_power: Vec<FieldElement> = set.iter().map(|x| x.pow(prime as i32 - 1)).collect();

    for elem in set_power {
        println!("{elem}");
    }

    Ok(())
}
