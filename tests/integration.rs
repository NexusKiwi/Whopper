use whopper::{Layer, Topping, Whopper, WhopperError, MAX_PICKLES};

#[test]
fn classic_has_buns_on_both_ends() {
    let w = Whopper::classic();
    assert_eq!(w.layers().first(), Some(&Layer::BottomBun));
    assert_eq!(w.layers().last(), Some(&Layer::TopBun));
}

#[test]
fn classic_has_exactly_one_patty() {
    assert_eq!(Whopper::classic().count(Topping::Patty), 1);
}

#[test]
fn cheese_is_an_upgrade() {
    assert!(!Whopper::classic().has(Topping::Cheese));
}

#[test]
fn no_onions_means_no_onions() {
    let w = Whopper::builder().without(Topping::Onion).build().unwrap();
    assert!(!w.has(Topping::Onion));
}

#[test]
fn no_patty_is_a_salad_sandwich() {
    let r = Whopper::builder().without(Topping::Patty).build();
    assert_eq!(r, Err(WhopperError::NoPatty));
}

#[test]
fn pickle_overflow_is_caught() {
    let mut b = Whopper::builder();
    for _ in 0..MAX_PICKLES {
        b = b.extra(Topping::Pickle);
    }
    assert!(matches!(b.build(), Err(WhopperError::PickleOverflow(_))));
}

#[test]
fn burgers_have_a_height_limit() {
    let mut b = Whopper::builder();
    for _ in 0..30 {
        b = b.extra(Topping::Lettuce);
    }
    assert!(matches!(b.build(), Err(WhopperError::TooTall { .. })));
}

#[test]
fn topping_parser_handles_plurals() {
    assert_eq!(Topping::parse("onions"), Some(Topping::Onion));
    assert_eq!(Topping::parse("Pickles"), Some(Topping::Pickle));
    assert_eq!(Topping::parse("tomatoes"), Some(Topping::Tomato));
    assert_eq!(Topping::parse("pineapple"), None); // absolutely not
}

#[test]
fn assembly_is_deterministic() {
    assert_eq!(Whopper::classic(), Whopper::classic());
}
