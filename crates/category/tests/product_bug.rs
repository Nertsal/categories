use category::{axioms, Bindings};
use category::{prelude::*, Equality};

use std::fmt::Debug;

#[test]
fn test_bug() {
    // Get rules
    let rule_product = axioms::rule_product::<&str>().unwrap();

    // Build the initial category
    let mut category = Category::new();

    let object_a = category.new_object(Object {
        tags: vec![],
        inner: (),
    });

    // Test product AxA
    let result = category.apply_rule(&rule_product, Bindings::new(), |_| (), |_, _| (), |_| ());
    assert!(result.1);
    print_category(&category);
    assert_eq!(2, category.objects.len());
    assert_eq!(6, category.morphisms.len());
    assert_eq!(6, category.equalities.len());

    // Undo
    for action in result.0 {
        category.action_do(action);
    }
    assert_eq!(1, category.objects.len());
    assert_eq!(0, category.morphisms.len());
    assert_eq!(0, category.equalities.len());

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
            tags: vec![MorphismTag::Unique, MorphismTag::ProductP2],
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
            tags: vec![MorphismTag::ProductP1],
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

    // Objects
    //  A, 1, Ax1
    // Morphisms
    //  f: IDa, IDax1, g: A->1, p2: Ax1->1, m: A->Ax1, p1: Ax1->A, m . p1: Ax1->A->Ax1, p2 . m . p1: Ax1->A->Ax1->1
    // Equalities
    //  Ax1->A->Ax1->1 = Ax1->1 (p2 . m . p1 = p2)
    //  Ax1->A->Ax1->A = Ax1->A (p1 . m . p1 = p1)
    //  IDa = A->Ax1->A (f = p1 . m)
    //  A->1 = A->Ax1->1 (g = p2 . m)

    print_category(&category);
    assert_eq!(3, category.objects.len());
    assert_eq!(8, category.morphisms.len());
    assert_eq!(4, category.equalities.len());

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
