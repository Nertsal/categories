use category::{axioms, Bindings};
use category::{prelude::*, Equality};

use std::fmt::Debug;

#[test]
fn test_bug() {
    // Build the initial category
    let mut category = Category::new();

    let object_a = category.new_object(Object {
        tags: vec![],
        inner: (),
    });
    let object_1 = category.new_object(Object {
        tags: vec![ObjectTag::Terminal],
        inner: (),
    });
    let object_ax1 = category.new_object(Object {
        tags: vec![ObjectTag::Product(object_a, object_1)],
        inner: (),
    });
    let morphism_id_a = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_a,
                to: object_a,
            },
            tags: vec![MorphismTag::Identity(object_a)],
            inner: (),
        })
        .unwrap();
    let _morphism_id_ax1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_ax1,
                to: object_ax1,
            },
            tags: vec![MorphismTag::Identity(object_ax1)],
            inner: (),
        })
        .unwrap();
    let morphism_a_1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_a,
                to: object_1,
            },
            tags: vec![MorphismTag::Unique],
            inner: (),
        })
        .unwrap();
    let morphism_ax1_1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_ax1,
                to: object_1,
            },
            tags: vec![MorphismTag::Unique],
            inner: (),
        })
        .unwrap();
    let morphism_a_ax1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_a,
                to: object_ax1,
            },
            tags: vec![],
            inner: (),
        })
        .unwrap();
    let morphism_ax1_a = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_ax1,
                to: object_a,
            },
            tags: vec![],
            inner: (),
        })
        .unwrap();
    let morphism_ax1_ax1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_ax1,
                to: object_ax1,
            },
            tags: vec![MorphismTag::Composition {
                first: morphism_ax1_a,
                second: morphism_a_ax1,
            }],
            inner: (),
        })
        .unwrap();
    let _morphism_ax1_ax1_1 = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular {
                from: object_ax1,
                to: object_1,
            },
            tags: vec![MorphismTag::Composition {
                first: morphism_ax1_ax1,
                second: morphism_ax1_1,
            }],
            inner: (),
        })
        .unwrap();
    category.equalities.new_equality(
        Equality::new(
            vec![morphism_ax1_a, morphism_a_ax1, morphism_ax1_1],
            vec![morphism_ax1_1],
        )
        .unwrap(),
        (),
    );
    category.equalities.new_equality(
        Equality::new(
            vec![morphism_ax1_a],
            vec![morphism_ax1_a, morphism_a_ax1, morphism_ax1_a],
        )
        .unwrap(),
        (),
    );
    category.equalities.new_equality(
        Equality::new(vec![morphism_id_a], vec![morphism_a_ax1, morphism_ax1_a]).unwrap(),
        (),
    );
    category.equalities.new_equality(
        Equality::new(vec![morphism_a_1], vec![morphism_a_ax1, morphism_ax1_1]).unwrap(),
        (),
    );

    print_category(&category);
    assert_eq!(3, category.objects.len());
    assert_eq!(8, category.morphisms.len());
    assert_eq!(4, category.equalities.len());

    // Get rules
    let rule_product = axioms::rule_product::<&str>().unwrap();

    // Test bug
    let bindings = Bindings::from_objects([("A", object_a), ("B", object_1)]);
    let result = category.apply_rule(&rule_product, bindings, |_| (), |_, _| (), |_| ());
    assert!(result.1);

    print_category(&category);

    assert_eq!(3, category.objects.len());
    assert_eq!(8, category.morphisms.len());
    assert_eq!(5, category.equalities.len());
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
