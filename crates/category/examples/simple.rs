use category::prelude::*;
use category::{axioms, Bindings, CategoryBuilder};

use std::fmt::Debug;

fn main() {
    let mut category = CategoryBuilder::<_, _, _, &str>::new()
        .object("A", vec![], ()) // Object A
        .object("B", vec![], ()) // Object B
        .morphism("f", "A", "B", vec![], ()) // Morphism f A->B
        .build();

    let morphism_f = *category.morphisms.iter().next().unwrap().0;

    print_category(&category);

    // Construct identity morphisms for every object
    category.apply_rule(
        &axioms::rule_identity().unwrap(),
        Bindings::<&str>::new(),
        |_| (),
        |_, _| (),
        |_| (),
    );

    print_category(&category);

    // Compose our morphism f with the identity morphism
    let mut bindings = Bindings::new();
    bindings.bind_morphism("f", morphism_f); // "f" is from the rule
    category.apply_rule(
        &axioms::rule_composition().unwrap(),
        bindings,
        |_| (),
        |_, _| (),
        |_| (),
    );

    print_category(&category);
}

fn print_category<O: Debug, M: Debug, E: Debug>(category: &Category<O, M, E>) {
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
    for (equality, inner) in category.equalities.iter() {
        println!(
            "  {:?} = {:?}: {inner:?}",
            equality.left(),
            equality.right()
        );
    }
    println!("");
}
