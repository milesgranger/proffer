pub mod utilities;
use proffer::{Attribute, SrcCode};

#[test]
fn test_attribute_attr() {
    let ann = "#[attr]";
    let attribute = Attribute::from(ann);
    match &attribute {
        &Attribute::ItemAttr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Attribute::ItemAttr, got {:?}", ann),
    };
    assert_eq!(&attribute.generate(), ann);
}

#[test]
fn test_attribute_mod_attr() {
    let ann = "#![foo_attr]";
    let attribute = Attribute::from(ann);
    match &attribute {
        &Attribute::ScopeAttr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Attribute::ScopeAttr, got {:?}", ann),
    };
    assert_eq!(&attribute.generate(), ann);
}
