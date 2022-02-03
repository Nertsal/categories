use category::*;

fn main() {
    let mut category = Category::new();
    let a = category.new_object(Object { tags: vec![] });
    let b = category.new_object(Object { tags: vec![] });
    category.new_morphism(Morphism {
        connection: MorphismConnection::Regular { from: a, to: b },
        tags: vec![],
    });

    print_category_state(&category);

    category.apply_rule(&axioms::rule_identity::<String>().unwrap(), Bindings::new());

    print_category_state(&category);
}

fn print_category_state(category: &Category) {
    println!("Objects: ");
    for (id, object) in category.objects.iter() {
        println!(" {id:?} - {:?}", object);
    }

    println!("Morphisms: ");
    for (id, morphism) in category.morphisms.iter() {
        println!(" {id:?} - {:?}", morphism);
    }
}
