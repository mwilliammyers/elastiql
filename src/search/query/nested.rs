//! [Nested query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html)

use serde::{Deserialize, Serialize};

use super::super::query::CompoundQuery;
#[cfg(feature = "graphql")]
use super::super::query::CompoundQueryInput;

/// A [Nested query] wraps another query to search [nested] fields.
///
/// The [nested query] searches nested field objects as if they were indexed as
/// separate documents. If an object matches the search, the nested query
/// returns the root parent document.
///
/// [Nested query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[graphql(name = "NestedFilterInput")]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct NestedQueryInput {
    /// Path to the nested object to search.
    pub path: String,

    /// Query to run on nested objects in the path. If an object
    /// matches the search, the nested query returns the root parent document.
    ///
    /// You can search nested fields using dot notation that includes the
    /// complete path, such as `obj1.name`.
    ///
    /// Multi-level nesting is automatically supported, and detected, resulting
    /// in an inner nested query to automatically match the relevant nesting
    /// level, rather than root, if it exists within another nested query.
    pub query: CompoundQueryInput,

    /// Indicates whether to ignore an unmapped path and not return any
    /// documents instead of an error.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub ignore_unmapped: bool,
}

#[cfg(feature = "graphql")]
impl NestedQueryInput {
    /// Constructs a new `NestedQueryInput`.
    #[inline]
    pub fn new(
        path: impl Into<String>,
        query: impl Into<CompoundQueryInput>,
        ignore_unmapped: bool,
    ) -> NestedQueryInput {
        NestedQueryInput {
            path: path.into(),
            query: query.into(),
            ignore_unmapped,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<NestedQuery> for NestedQueryInput {
    #[inline]
    fn from(query: NestedQuery) -> Self {
        Self {
            path: query.path,
            query: query.query.into(),
            ignore_unmapped: query.ignore_unmapped,
        }
    }
}

/// A [Nested query] wraps another query to search [nested] fields.
///
/// The [nested query] searches nested field objects as if they were indexed as
/// separate documents. If an object matches the search, the nested query
/// returns the root parent document.
///
/// [Nested query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html
/// [nested]: https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "NestedFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct NestedQuery {
    /// Path to the nested object to search.
    pub path: String,

    /// Query to run on nested objects in the path. If an object
    /// matches the search, the nested query returns the root parent document.
    ///
    /// You can search nested fields using dot notation that includes the
    /// complete path, such as `obj1.name`.
    ///
    /// Multi-level nesting is automatically supported, and detected, resulting
    /// in an inner nested query to automatically match the relevant nesting
    /// level, rather than root, if it exists within another nested query.
    pub query: CompoundQuery,

    /// Indicates whether to ignore an unmapped path and not return any
    /// documents instead of an error.
    #[cfg_attr(feature = "builder", builder(default))]
    pub ignore_unmapped: bool,
}

impl NestedQuery {
    /// Constructs a new `NestedQuery`.
    #[inline]
    pub fn new(
        path: impl Into<String>,
        query: impl Into<CompoundQuery>,
        ignore_unmapped: bool,
    ) -> NestedQuery {
        NestedQuery {
            path: path.into(),
            query: query.into(),
            ignore_unmapped,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<NestedQueryInput> for NestedQuery {
    #[inline]
    fn from(input: NestedQueryInput) -> NestedQuery {
        NestedQuery {
            path: input.path,
            query: input.query.into(),
            ignore_unmapped: input.ignore_unmapped,
        }
    }
}
