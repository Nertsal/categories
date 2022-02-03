use ::category::CategoryBuilder;

use super::*;

pub fn fact_category() -> ::category::types::Category {
    CategoryBuilder::<Label>::new()
        .object("A", vec![])
        .object("B", vec![])
        .object("C", vec![])
        .build()
}

pub fn goal_category() -> ::category::types::Category {
    CategoryBuilder::<Label>::new()
        .object("A", vec![])
        .object("B", vec![])
        .object("C", vec![])
        .object("AxB", vec![ObjectTag::Product("A".into(), "B".into())])
        .object("BxC", vec![ObjectTag::Product("B".into(), "C".into())])
        .object(
            "(AxB)xC",
            vec![ObjectTag::Product("AxB".into(), "C".into())],
        )
        .object(
            "Ax(BxC)",
            vec![ObjectTag::Product("A".into(), "BxC".into())],
        )
        .isomorphism("", "Ax(BxC)", "(AxB)xC", vec![])
        .build()
}
