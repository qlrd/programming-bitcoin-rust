use crate::primitives::field_element::FieldElement;

#[derive(Debgug, Clone)]
pub struct Signature {
    r: FieldElement,
    s: FieldElement,
}

impl Signature {}
