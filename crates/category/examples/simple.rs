use category::*;

fn main() {
    let mut category = Category::new();
    let a = category.new_object(Object {
        tags: vec![],
        inner: (),
    });
    let b = category.new_object(Object {
        tags: vec![],
        inner: (),
    });
    category.new_morphism(Morphism {
        connection: MorphismConnection::Regular { from: a, to: b },
        tags: vec![],
        inner: (),
    });

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
