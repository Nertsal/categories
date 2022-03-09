use category::constraint::ConstraintsBuilder;
use category::prelude::*;
use category::{Bindings, CategoryBuilder};

#[test]
fn test_find() {
    let category = CategoryBuilder::<(), (), (), &str>::new()
        .object("A", vec![], ())
        .object("B", vec![], ())
        .object("AxB", vec![ObjectTag::Product("A", "B")], ())
        .morphism("p1", "AxB", "A", vec![], ())
        .morphism("p2", "AxB", "B", vec![], ())
        .morphism("id", "AxB", "AxB", vec![MorphismTag::Identity("AxB")], ())
        .build();

    let constraints = ConstraintsBuilder::<&str>::new()
        .object("A", vec![])
        .object("B", vec![])
        .object("AxB", vec![ObjectTag::Product("A", "B")])
        .morphism("p1", "AxB", "A", vec![])
        .morphism("p2", "AxB", "B", vec![])
        .object("C", vec![])
        .morphism("f", "C", "A", vec![])
        .morphism("g", "C", "B", vec![])
        .morphism("m", "C", "AxB", vec![])
        .equality(vec!["m", "p1"], vec!["f"])
        .equality(vec!["m", "p2"], vec!["g"])
        .build();
    let candidates = category
        .find_candidates(&constraints, &Bindings::new())
        .unwrap()
        .collect::<Vec<_>>();

    println!("Candidates for:");
    println!("  {constraints:?}");
    println!("are:");
    for (i, candidate) in candidates.iter().enumerate() {
        println!("{i:4}) {candidate:?}");
    }

    assert_eq!(candidates.len(), 1);
}
