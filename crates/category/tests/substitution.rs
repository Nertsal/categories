use category::{axioms, Bindings};
use category::{prelude::*, CategoryBuilder};

use std::fmt::Debug;

#[test]
fn test_substitution() {
    // Build the initial category
    let mut category = CategoryBuilder::<_, _, _, &str>::new()
        .object("A", vec![], ())
        .object("B", vec![], ())
        .morphism("f", "A", "B", vec![], ())
        .morphism("id", "A", "A", vec![], ())
        .morphism(
            "f.id",
            "A",
            "B",
            vec![MorphismTag::Composition {
                first: "id",
                second: "f",
            }],
            (),
        )
        .morphism("m", "A", "B", vec![MorphismTag::Unique], ())
        .build();

    print_category(&category);

    // Make sure the build is correct
    assert_eq!(2, category.objects.len());
    assert_eq!(4, category.morphisms.len());
    assert_eq!(0, category.equalities.len());

    // Get rules
    let rule_unique = axioms::rule_unique::<&str>().unwrap();

    // Apply unique rule
    category.apply_rule(&rule_unique, Bindings::new(), |_| (), |_, _| (), |_| ());
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
