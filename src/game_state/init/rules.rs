use super::*;

pub fn default_rules(geng: &Geng, assets: &Rc<Assets>) -> Rules {
    vec![
        // Identity: forall (object A) exists (morphism id A->A [Identity])
        RuleBuilder::new()
            .forall(ConstraintsBuilder::new().object("A", None))
            .exists(ConstraintsBuilder::new().morphism(
                "id",
                "A",
                "A",
                Some(MorphismTag::Identity(Some("A"))),
            ))
            .build(geng, assets),
        // Composition: forall (morphism f A->B, morphism g B->C) exists (morphism g.f A->C [Composition f g])
        RuleBuilder::new()
            .forall(
                ConstraintsBuilder::new()
                    .morphism("f", "A", "B", None)
                    .morphism("g", "B", "C", None),
            )
            .exists(ConstraintsBuilder::new().morphism(
                "g.f",
                "A",
                "C",
                Some(MorphismTag::Composition {
                    first: Some("f"),
                    second: Some("g"),
                }),
            ))
            .build(geng, assets),
        // Product: forall (object A, object B)
        //          exists (object AxB [Product A B])
        //          exists (morphism _ AxB->A, morphism _ AxB->B)
        //          forall (object C, morphism f C->A, morphism g C->B)
        //          exists (morphism m C->AxB [Unique])
        //          forall (morphism m' C->AxB)
        //                  m = m'
        RuleBuilder::new()
            .forall(
                ConstraintsBuilder::new()
                    .object("A", None)
                    .object("B", None),
            )
            .exists(
                ConstraintsBuilder::new()
                    .object("AxB", Some(ObjectTag::Product(Some("A"), Some("B")))),
            )
            .exists(
                ConstraintsBuilder::new()
                    .morphism("fst", "AxB", "A", None)
                    .morphism("snd", "AxB", "B", None),
            )
            .forall(
                ConstraintsBuilder::new()
                    .object("C", None)
                    .morphism("f", "C", "A", None)
                    .morphism("g", "C", "B", None),
            )
            .exists(ConstraintsBuilder::new().morphism("m", "C", "AxB", Some(MorphismTag::Unique)))
            // .forall(ConstraintsBuilder::new().morphism("m'", "C", "AxB", None))
            // TODO: m = m'
            .build(geng, assets),
        // Isomorphism: forall (morphism f A->B, morphism g B->A) // TODO: f.g = id_a, g.f = id_b
        //              exists (morphism _ A<=>B [Isomorphism f g])
        RuleBuilder::new()
            .forall(
                ConstraintsBuilder::new()
                    .morphism("f", "A", "B", None)
                    .morphism("g", "B", "A", None),
            )
            .exists(ConstraintsBuilder::new().morphism(
                "",
                "A",
                "B",
                Some(MorphismTag::Isomorphism(Some("f"), Some("g"))),
            ))
            .build(geng, assets),
    ]
}
