pub struct MatchingAttribute {
    pub name: &'static str,
    pub value: &'static str,
}

lazy_static! {
    pub static ref MATCHING_ATTRIBUTES: Vec<MatchingAttribute> = vec![MatchingAttribute {
        name: "name",
        value: "STATEMENTS_OF_OPERATIONS",
    }];
}
