use category::*;

fn main() {
    let mut category = Category::new();
    let a = category.new_object(Object { tags: vec![] });
    let b = category.new_object(Object { tags: vec![] });
    let f = category
        .new_morphism(Morphism {
            connection: MorphismConnection::Regular { from: a, to: b },
            tags: vec![],
        })
        .unwrap();

    print_category_state(&category);

    // Construct identity morphisms for every object
    category.apply_rule(&axioms::rule_identity().unwrap(), Bindings::<String>::new());

    print_category_state(&category);

    // Compose our morphism f with the identity morphism
    let mut bindings = Bindings::new();
    bindings.bind_morphism("f", f);
    category.apply_rule(&axioms::rule_composition().unwrap(), bindings);

    print_category_state(&category);
}

fn print_category_state(category: &Category) {
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
