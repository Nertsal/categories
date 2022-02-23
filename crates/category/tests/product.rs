use category::constraint::ConstraintsBuilder;
use category::prelude::*;
use category::{axioms, Bindings};

use std::fmt::Debug;

#[test]
fn test_product() {
    // Build the initial category
    let mut category = Category::new();

    let object_a = category.new_object(Object {
        tags: vec![],
        inner: (),
    });
    let object_b = category.new_object(Object {
        tags: vec![],
        inner: (),
    });
    let bindings = Bindings::from_objects(vec![("A", object_a), ("B", object_b)]);

    print_category(&category);

    // Make sure the build is correct
    assert_eq!(2, category.objects.len());
    assert_eq!(0, category.morphisms.len());
    assert_eq!(0, category.equalities.all_equalities().count());

    // Get rules
    let rule_identity = axioms::rule_identity::<&str>().unwrap();
    let rule_product = axioms::rule_product::<&str>().unwrap();
    let rule_composition = axioms::rule_composition::<&str>().unwrap();

    // Apply product rule
    category.apply_rule(&rule_product, bindings.clone(), |_| (), |_, _| ());
    print_category(&category);
    assert_eq!(3, category.objects.len());
    assert_eq!(3, category.morphisms.len());
    assert_eq!(0, category.equalities.all_equalities().count());

    // Find morphisms f: Identity(AxB), g: AxB->A
    let constraints = ConstraintsBuilder::new()
        .object("AxB", vec![ObjectTag::Product("A", "B")])
        .morphism("m", "AxB", "AxB", vec![])
        .build();
    let morphism_f = category
        .find_candidates(&constraints, &bindings)
        .unwrap()
        .next()
        .unwrap()
        .get_morphism(&"m")
        .unwrap();
    let constraints = ConstraintsBuilder::new()
        .object("AxB", vec![ObjectTag::Product("A", "B")])
        .morphism("m", "AxB", "A", vec![])
        .build();
    let morphism_g = category
        .find_candidates(&constraints, &bindings)
        .unwrap()
        .next()
        .unwrap()
        .get_morphism(&"m")
        .unwrap();

    println!("f = {morphism_f:?}");
    println!("g = {morphism_g:?}");

    // Apply composition rule
    category.apply_rule(
        &rule_composition,
        Bindings::from_morphisms(vec![("f", morphism_f), ("g", morphism_g)]),
        |_| (),
        |_, _| (),
    );
    print_category(&category);
    assert_eq!(3, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(0, category.equalities.all_equalities().count());

    // Apply product rule
    category.apply_rule(&rule_product, bindings, |_| (), |_, _| ());
    print_category(&category);
    assert_eq!(3, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(0, category.equalities.all_equalities().count());
}

fn print_category<O: Debug, M: Debug>(category: &Category<O, M>) {
    println!("\n----- Category -----");
    println!("Objects:");
    for (id, object) in category.objects.iter() {
        println!("{:4} - {:?}", id.raw(), object)
    }
    println!("Morphisms:");
    for (id, morphism) in category.morphisms.iter() {
        println!("{:4} - {:?}", id.raw(), morphism)
    }
    println!("Equalities:");
    for equality in category.equalities.all_equalities() {
        println!("  {:?} = {:?}", equality.left(), equality.right());
    }
    println!("");
}
