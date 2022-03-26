use category::prelude::*;
use category::{axioms, Bindings, CategoryBuilder};

use std::fmt::Debug;

#[test]
fn test_bug() {
    // Get rules
    let rule_unique = axioms::rule_unique::<&str>().unwrap();

    // Build the initial category
    let mut category = CategoryBuilder::<_, _, _, &str>::new()
        .object("A", vec![], ())
        .object("B", vec![], ())
        .morphism("id", "A", "A", vec![MorphismTag::Identity("A")], ())
        .morphism("f", "A", "B", vec![], ())
        .morphism("g", "B", "A", vec![], ())
        .equality(vec!["f", "g"], vec!["id"], ())
        .morphism("m", "B", "A", vec![MorphismTag::Unique], ())
        .build();

    print_category(&category);
    assert_eq!(2, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(1, category.equalities.len());

    // Substitute g=m
    let result = category.apply_rule(&rule_unique, Bindings::new(), |_| (), |_, _| (), |_| ());
    assert!(result.1);

    print_category(&category);
    assert_eq!(2, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(2, category.equalities.len());

    // Test undo
    let result = result
        .0
        .into_iter()
        .flat_map(|undo| category.action_do(undo))
        .collect::<Vec<_>>();

    print_category(&category);
    assert_eq!(2, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(1, category.equalities.len());

    // Test redo
    result.into_iter().for_each(|redo| {
        category.action_do(redo);
    });

    print_category(&category);
    assert_eq!(2, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(2, category.equalities.len());
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
