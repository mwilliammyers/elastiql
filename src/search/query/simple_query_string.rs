//! [Simple query string query](elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html)

use serde::{Deserialize, Serialize};

// TODO: add additional options
/// A [Simple query string] returns documents based on a provided query string,
/// using a parser with a limited but fault-tolerant syntax.
///
/// This query uses a [simple syntax] to parse and split the provided query
/// string into terms based on special operators. The query then analyzes each
/// term independently before returning matching documents.
///
/// While its syntax is more limited than the [`query_string` query], the
/// simple_query_string query does not return errors for invalid syntax.
/// Instead, it ignores any invalid parts of the query string.
///
/// [Simple query string]: elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html
/// [simple syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax
/// [`query_string` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[graphql(name = "SimpleQueryStringFilterInput")]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct SimpleQueryStringQueryInput {
    /// The name of the fields to query.
    ///
    /// Defaults to all field that have full text search enabled.
    ///
    /// Accepts wildcard expressions. You also can boost relevance scores for
    /// matches to particular fields using a caret (`^`) notation. See
    /// [Wildcards and per-field boosts in the fields parameter] for examples.
    ///
    /// [Wildcards and per-field boosts in the fields parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-boost
    #[graphql(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    pub query: String,
}

#[cfg(feature = "graphql")]
impl SimpleQueryStringQueryInput {
    /// Constructs a new `SimpleQueryStringQueryInput`.
    #[inline]
    pub fn new<I, T>(fields: I, query: impl Into<String>) -> SimpleQueryStringQueryInput
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        SimpleQueryStringQueryInput {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            query: query.into(),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<SimpleQueryStringQuery> for SimpleQueryStringQueryInput {
    #[inline]
    fn from(query: SimpleQueryStringQuery) -> Self {
        Self {
            fields: query.fields,
            query: query.query,
        }
    }
}

// TODO: add additional options
/// A [Simple query string] returns documents based on a provided query string,
/// using a parser with a limited but fault-tolerant syntax.
///
/// This query uses a [simple syntax] to parse and split the provided query
/// string into terms based on special operators. The query then analyzes each
/// term independently before returning matching documents.
///
/// While its syntax is more limited than the [`query_string` query], the
/// simple_query_string query does not return errors for invalid syntax.
/// Instead, it ignores any invalid parts of the query string.
///
/// [Simple query string]: elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html
/// [simple syntax]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax
/// [`query_string` query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "SimpleQueryStringFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct SimpleQueryStringQuery {
    /// The name of the fields to query.
    ///
    /// Defaults to all field that have full text search enabled.
    ///
    /// Accepts wildcard expressions. You also can boost relevance scores for
    /// matches to particular fields using a caret (`^`) notation. See
    /// [Wildcards and per-field boosts in the fields parameter] for examples.
    ///
    /// [Wildcards and per-field boosts in the fields parameter]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-boost
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,

    /// The query to run in the [simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    pub query: String,
}

impl SimpleQueryStringQuery {
    /// Constructs a new `SimpleQueryStringQuery`.
    #[inline]
    pub fn new<I, T>(fields: I, query: impl Into<String>) -> SimpleQueryStringQuery
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        SimpleQueryStringQuery {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            query: query.into(),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<SimpleQueryStringQueryInput> for SimpleQueryStringQuery {
    #[inline]
    fn from(input: SimpleQueryStringQueryInput) -> SimpleQueryStringQuery {
        SimpleQueryStringQuery {
            fields: input.fields,
            query: input.query,
        }
    }
}
