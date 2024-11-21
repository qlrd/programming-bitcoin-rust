use programming_bitcoin_in_rust::primitives::field_element;
use programming_bitcoin_in_rust::primitives::point;

#[cfg(test)]
mod point_test {
    use super::*;
    use field_element::FieldElement;
    use point::Point;

    #[test]
    fn test_new() {
        let fe_x = FieldElement::new("1", "13").unwrap();
        let fe_y = FieldElement::new("3", "13").unwrap();
        let fe_a = FieldElement::new("2", "13").unwrap();
        let fe_b = FieldElement::new("6", "13").unwrap();

        let _ = Point::new(Some(fe_x), Some(fe_y), fe_a, fe_b).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Point(FiniteElement_107187(2), FiniteElement_107187(3))_<a=FiniteElement_107187(2), b=FiniteElement_107187(6)> is not on the curve"
    )]
    fn test_new_fail_not_on_curve() {
        let fe_x = FieldElement::new("2", "01a2b3").unwrap();
        let fe_y = FieldElement::new("3", "01a2b3").unwrap();
        let fe_a = FieldElement::new("2", "01a2b3").unwrap();
        let fe_b = FieldElement::new("6", "01a2b3").unwrap();
        let _ = Point::new(Some(fe_x), Some(fe_y), fe_a, fe_b).unwrap();
    }

    #[test]
    fn test_eq() {
        let fe_x = FieldElement::new("1", "13").unwrap();
        let fe_y = FieldElement::new("3", "13").unwrap();
        let fe_a = FieldElement::new("2", "13").unwrap();
        let fe_b = FieldElement::new("6", "13").unwrap();

        let fe_x2 = FieldElement::new("1", "13").unwrap();
        let fe_y2 = FieldElement::new("3", "13").unwrap();
        let fe_a2 = FieldElement::new("2", "13").unwrap();
        let fe_b2 = FieldElement::new("6", "13").unwrap();

        let p1 = Point::new(Some(fe_x), Some(fe_y), fe_a, fe_b).unwrap();
        let p2 = Point::new(Some(fe_x2), Some(fe_y2), fe_a2, fe_b2).unwrap();
        assert!(p1 == p2)
    }
}
