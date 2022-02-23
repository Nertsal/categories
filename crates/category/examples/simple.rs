use category::prelude::*;

use std::fmt::Debug;

fn main() {
    // let category = CategoryBuilder::<(), (), &str>::new()
    //     .object("A", vec![], ()) // Object A
    //     .object("B", vec![], ()) // Object B
    //     .morphism("f", "A", "B", vec![], ()) // Morphism f A->B
    //     .build();

    // print_category_state(&category);

    // // Construct identity morphisms for every object
    // category.apply_rule(
    //     &axioms::rule_identity().unwrap(),
    //     Bindings::<String>::new(),
    //     |_| (),
    //     |_, _| (),
    // );

    // print_category_state(&category);

    // // Compose our morphism f with the identity morphism
    // let mut bindings = Bindings::new();
    // bindings.bind_morphism("f", f);
    // category.apply_rule(&axioms::rule_composition().unwrap(), bindings);

    // print_category_state(&category);
}

fn print_category_state<O: Debug, M: Debug>(category: &Category<O, M>) {
    println!("\nPrinting category...");
    println!("Objects: ");
    for (id, object) in category.objects.iter() {
        println!(" {id:?} - {:?}", object);
    }

    println!("Morphisms: ");
    for (id, morphism) in category.morphisms.iter() {
        println!(" {id:?} - {:?}", morphism);
    }
}
