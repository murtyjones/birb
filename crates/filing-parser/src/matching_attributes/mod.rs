pub struct MatchingAttribute {
    pub name: &'static str,
    pub value: &'static str,
}

pub fn get_matching_attrs() -> Vec<MatchingAttribute> {
    vec![
        MatchingAttribute {
            name: "name",
            value: "STATEMENTS_OF_OPERATIONS",
        },
        MatchingAttribute {
            // pertains to 0001185185-16-005747.html
            name: "name",
            value: "Operations",
        },
    ]
}
