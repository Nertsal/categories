use super::*;

pub fn default_rules(geng: &Geng, assets: &Rc<Assets>) -> Rules {
    vec![
        // Identity: forall (object A) exists (morphism id A->A [Identity])
        RuleBuilder::new()
            .forall(ConstraintsBuilder::new().object("A", vec![]))
            .exists(ConstraintsBuilder::new().morphism(
                "id",
                "A",
                "A",
                vec![MorphismTag::Identity(Some("A"))],
            ))
            .build(geng, assets),
        // Composition: forall (morphism f A->B, morphism g B->C) exists (morphism g.f A->C [Composition f g])
        RuleBuilder::new()
            .forall(
                ConstraintsBuilder::new()
                    .morphism("f", "A", "B", vec![])
                    .morphism("g", "B", "C", vec![]),
            )
            .exists(ConstraintsBuilder::new().morphism(
                "g.f",
                "A",
                "C",
                vec![MorphismTag::Composition {
                    first: Some("f"),
                    second: Some("g"),
                }],
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
                    .object("A", vec![])
                    .object("B", vec![]),
            )
            .exists(
                ConstraintsBuilder::new()
                    .object("AxB", vec![ObjectTag::Product(Some("A"), Some("B"))]),
            )
            .exists(
                ConstraintsBuilder::new()
                    .morphism("fst", "AxB", "A", vec![])
                    .morphism("snd", "AxB", "B", vec![]),
            )
            .forall(
                ConstraintsBuilder::new()
                    .object("C", vec![])
                    .morphism("f", "C", "A", vec![])
                    .morphism("g", "C", "B", vec![]),
            )
            .exists(ConstraintsBuilder::new().morphism("m", "C", "AxB", vec![MorphismTag::Unique]))
            // .forall(ConstraintsBuilder::new().morphism("m'", "C", "AxB", vec![]))
            // TODO: m = m'
            .build(geng, assets),
        // Isomorphism: forall (morphism f A->B, morphism g B->A) // TODO: f.g = id_a, g.f = id_b
        //              exists (morphism _ A<=>B [Isomorphism f g])
        RuleBuilder::new()
            .forall(
                ConstraintsBuilder::new()
                    .morphism("f", "A", "B", vec![])
                    .morphism("g", "B", "A", vec![]),
            )
            .exists(ConstraintsBuilder::new().morphism(
                "",
                "A",
                "B",
                vec![MorphismTag::Isomorphism(Some("f"), Some("g"))],
            ))
            .build(geng, assets),
    ]
}
