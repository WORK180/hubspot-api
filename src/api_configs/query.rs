/// Check if query has begun.
pub fn query_begun_check(checkpoint: bool) -> (String, bool) {
    if checkpoint {
        ("&".to_string(), checkpoint)
    } else {
        ("?".to_string(), true)
    }
}

/// Build the query for hubspot paging.
pub fn build_paging_query(limit: Option<i32>, after: Option<&str>) -> (String, bool) {
    let mut query_begun = false;

    let limit_query = match limit {
        Some(limit) => {
            query_begun = true;
            format!("?limit={}", limit)
        }
        None => String::new(),
    };

    let after_query = match after {
        Some(after) => {
            let query_check = query_begun_check(query_begun);
            query_begun = query_check.1;
            format!("{}after={}", query_check.0, after)
        }
        None => String::new(),
    };

    (format!("{}{}", limit_query, after_query), query_begun)
}

/// Build a query string from properties, properties_with_history, associations, and archived
pub fn build_query_string(
    query_already_begun: bool,
    properties: &[&str],
    properties_with_history: &[&str],
    associations: &[&str],
    archived: bool,
) -> String {
    let mut query_begun = query_already_begun;

    let property_query = if properties.is_empty() {
        String::new()
    } else {
        let query_check = query_begun_check(query_begun);
        query_begun = query_check.1;
        format!("{}properties={}", query_check.0, properties.join(","))
    };
    let properties_with_history_query = if properties_with_history.is_empty() {
        String::new()
    } else {
        let query_check = query_begun_check(query_begun);
        query_begun = query_check.1;
        format!(
            "{}propertiesWithHistory={}",
            query_check.0,
            properties_with_history.join(",")
        )
    };
    let associations_query = if associations.is_empty() {
        String::new()
    } else {
        let query_check = query_begun_check(query_begun);
        query_begun = query_check.1;
        format!("{}associations={}", query_check.0, associations.join(","))
    };
    let archived_query = if query_begun {
        format!("&archived={}", archived)
    } else {
        format!("?archived={}", archived)
    };

    format!("{property_query}{properties_with_history_query}{associations_query}{archived_query}")
}
