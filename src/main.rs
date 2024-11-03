mod field_element; // Import the field_element module

use field_element::FieldElement; // Bring FieldElement into scope

fn main() {

    match FieldElement::new(2, 7) {
        Ok(fe) => println!("{}", fe), // Prints "FieldElement_7(2)"
        Err(e) => println!("Error: {}", e),
    }

    let fe1 = FieldElement::new(2, 7).unwrap();
    let fe2 = FieldElement::new(2, 7).unwrap();
    let fe3 = FieldElement::new(3, 7).unwrap();

    // Equality check
    println!("{} == {}: {}", fe1, fe2, fe1 == fe2); // Prints true
    println!("{} == {}: {}", fe1, fe3, fe1 == fe3); // Prints false

    // Inequality check
    println!("{} != {}: {}", fe1, fe2, fe1 != fe2); // Prints false
    println!("{} != {}: {}", fe1, fe3, fe1 != fe3); // Prints true
}
