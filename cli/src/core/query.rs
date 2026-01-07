use std::collections::HashMap;

/// A parsed search query with structured filters and free-text terms.
#[derive(Debug, Default)]
pub struct Query {
    /// Key-value filters (e.g., status:active, priority:high)
    pub filters: HashMap<String, Vec<String>>,
    /// Free-text search terms
    pub terms: Vec<String>,
}

impl Query {
    /// Parse a query string into structured filters and free-text terms.
    ///
    /// Syntax:
    /// - `key:value` → adds to filters
    /// - `bare word` → adds to terms
    /// - Multiple values for same key: `tags:work tags:personal`
    pub fn parse(input: &str) -> Self {
        let mut query = Query::default();

        for token in input.split_whitespace() {
            if let Some((key, value)) = token.split_once(':') {
                let key = key.to_lowercase();
                let value = value.to_lowercase();
                if !value.is_empty() {
                    query.filters.entry(key).or_default().push(value);
                }
            } else {
                query.terms.push(token.to_lowercase());
            }
        }

        query
    }

    /// Check if query is empty (no filters or terms).
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty() && self.terms.is_empty()
    }
}

/// Possible values a field can have.
#[derive(Debug)]
pub enum FieldValue {
    /// Single string value
    Single(String),
    /// Multiple string values (e.g., tags)
    Multiple(Vec<String>),
}

impl FieldValue {
    /// Check if this field value matches the given filter value.
    pub fn matches(&self, filter: &str) -> bool {
        match self {
            FieldValue::Single(v) => v.to_lowercase().contains(filter),
            FieldValue::Multiple(vs) => vs.iter().any(|v| v.to_lowercase().contains(filter)),
        }
    }
}

/// A trait for items that can be matched against a query.
pub trait Queryable {
    /// Get the value(s) for a given filter key.
    fn get_field(&self, key: &str) -> Option<FieldValue>;

    /// Get searchable text for free-text matching.
    fn searchable_text(&self) -> String;
}

/// Check if an item matches the query.
pub fn matches<T: Queryable>(item: &T, query: &Query) -> bool {
    // Check all filters
    for (key, values) in &query.filters {
        match item.get_field(key) {
            Some(field) => {
                // All filter values for this key must match
                if !values.iter().all(|v| field.matches(v)) {
                    return false;
                }
            }
            None => return false,
        }
    }

    // Check free-text terms
    if !query.terms.is_empty() {
        let haystack = item.searchable_text().to_lowercase();
        if !query.terms.iter().all(|term| haystack.contains(term)) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let q = Query::parse("");
        assert!(q.is_empty());
    }

    #[test]
    fn parse_filters_only() {
        let q = Query::parse("status:active priority:high");
        assert_eq!(q.filters.get("status"), Some(&vec!["active".to_string()]));
        assert_eq!(q.filters.get("priority"), Some(&vec!["high".to_string()]));
        assert!(q.terms.is_empty());
    }

    #[test]
    fn parse_terms_only() {
        let q = Query::parse("meeting notes");
        assert!(q.filters.is_empty());
        assert_eq!(q.terms, vec!["meeting", "notes"]);
    }

    #[test]
    fn parse_mixed() {
        let q = Query::parse("status:active quarterly review");
        assert_eq!(q.filters.get("status"), Some(&vec!["active".to_string()]));
        assert_eq!(q.terms, vec!["quarterly", "review"]);
    }

    #[test]
    fn parse_multiple_same_key() {
        let q = Query::parse("tags:work tags:important");
        assert_eq!(
            q.filters.get("tags"),
            Some(&vec!["work".to_string(), "important".to_string()])
        );
    }

    #[test]
    fn field_value_single_match() {
        let v = FieldValue::Single("active".to_string());
        assert!(v.matches("act"));
        assert!(v.matches("active"));
        assert!(!v.matches("draft"));
    }

    #[test]
    fn field_value_multiple_match() {
        let v = FieldValue::Multiple(vec!["work".to_string(), "important".to_string()]);
        assert!(v.matches("work"));
        assert!(v.matches("important"));
        assert!(!v.matches("personal"));
    }
}
