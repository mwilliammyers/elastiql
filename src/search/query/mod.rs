//! Elasticsearch [Query DSL] types.
//!
//! [Query DSL]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html

use std::default::Default;

use serde::{Deserialize, Serialize};

pub use self::{
    exists::*, match_::*, nested::*, prefix::*, query_string::*, range::*, regexp::*,
    simple_query_string::*, term::*, terms::*,
};

mod exists;
mod match_;
mod nested;
mod prefix;
mod query_string;
mod range;
mod regexp;
mod simple_query_string;
mod term;
mod terms;

// TODO: make this file smaller!

/// [Compound queries] wrap other compound or leaf queries, either to combine
/// their results and scores, to change their behavior, or to switch from query
/// to filter context.
///
/// [Compound queries]: https://www.elastic.co/guide/en/elasticsearch/reference/current/compound-queries.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Default, Clone, Debug)]
#[graphql(name = "CompoundFilterInput")]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct CompoundQueryInput {
    /// The default query for combining multiple leaf or compound query clauses,
    /// as must, should, must_not, or filter clauses. The must and should
    /// clauses have their scores combined — the more matching clauses, the
    /// better — while the must_not and filter clauses are executed in filter
    /// context.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, rename = "bool", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanQueryInput>,
}

#[cfg(feature = "graphql")]
impl CompoundQueryInput {
    /// Returns `true` if this `CompoundQueryInput` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.boolean
            .as_ref()
            .map_or_else(|| true, |filter| filter.is_empty())
    }

    /// Appends a `filter` on to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<QueryInput>) {
        if let Some(ref mut boolean) = self.boolean {
            boolean.push(filter)
        } else {
            self.boolean = Some(BooleanQueryInput {
                must: vec![],
                filter: vec![filter.into()],
                should: vec![],
                must_not: vec![],
                minimum_should_match: None,
                boost: None,
            })
        }
    }
}

#[cfg(feature = "graphql")]
impl From<Option<CompoundQueryInput>> for CompoundQueryInput {
    #[inline]
    fn from(filter: Option<CompoundQueryInput>) -> CompoundQueryInput {
        filter.unwrap_or_default()
    }
}

#[cfg(feature = "graphql")]
impl<T: Into<BooleanQueryInput>> From<T> for CompoundQueryInput {
    #[inline]
    fn from(filter: T) -> CompoundQueryInput {
        CompoundQueryInput {
            boolean: Some(filter.into()),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<CompoundQuery> for CompoundQueryInput {
    #[inline]
    fn from(query: CompoundQuery) -> Self {
        Self {
            boolean: query.boolean.map(Into::into),
        }
    }
}

/// [Compound queries] wrap other compound or leaf queries, either to combine
/// their results and scores, to change their behavior, or to switch from query
/// to filter context.
///
/// [Compound queries]: https://www.elastic.co/guide/en/elasticsearch/reference/current/compound-queries.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "CompoundFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct CompoundQuery {
    /// The default query for combining multiple leaf or compound query clauses,
    /// as must, should, must_not, or filter clauses. The must and should
    /// clauses have their scores combined — the more matching clauses, the
    /// better — while the must_not and filter clauses are executed in filter
    /// context.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, rename = "bool", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanQuery>,
}

impl CompoundQuery {
    /// Returns `true` if this `CompoundQuery` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.boolean
            .as_ref()
            .map_or_else(|| true, |filter| filter.is_empty())
    }

    /// Appends a `filter` on to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<Query>) {
        if let Some(ref mut boolean) = self.boolean {
            boolean.push(filter)
        } else {
            self.boolean = Some(BooleanQuery {
                must: vec![],
                filter: vec![filter.into()],
                should: vec![],
                must_not: vec![],
                minimum_should_match: None,
                boost: None,
            })
        }
    }
}

impl<T: Into<BooleanQuery>> From<T> for CompoundQuery {
    #[inline]
    fn from(filter: T) -> CompoundQuery {
        CompoundQuery {
            boolean: Some(filter.into()),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<CompoundQueryInput> for CompoundQuery {
    #[inline]
    fn from(input: CompoundQueryInput) -> CompoundQuery {
        CompoundQuery {
            boolean: Some(input.boolean.unwrap_or_default().into()),
        }
    }
}

/// A [query] that matches documents matching boolean combinations of other
/// queries. It is built using one or more boolean clauses, each clause with a
/// typed occurrence.
///
/// [query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Default, Clone, Debug)]
#[graphql(name = "BooleanFilterInput")]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct BooleanQueryInput {
    /// The clause (query) must appear in matching documents and will
    /// contribute to the score of this query.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must: Vec<QueryInput>,

    /// The clause (query) must appear in matching documents. However unlike
    /// must, the score of the query will be ignored. Query clauses are executed
    /// in [filter context], meaning that scoring is ignored and clauses are
    /// considered for caching.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<QueryInput>,

    /// The clause (query) should appear in the matching document.
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub should: Vec<QueryInput>,

    /// The clause (query) must not appear in the matching documents. Clauses
    /// are executed in [filter context] meaning that scoring is ignored and
    /// clauses are considered for caching. Because scoring is ignored, a score
    /// of 0 for all documents is returned.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[graphql(default)]
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_not: Vec<QueryInput>,

    /// [Controls] how many optional (`should`) parameters must match.
    ///
    /// | Example       | Description                                                                                                                                                                                   |
    /// |---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `3`           | *Fixed value* regardless of the number of optional clauses                                                                                                                                    |
    /// | `-2`          | Total number of optional clauses, *minus* this number should be mandatory                                                                                                                     |
    /// | `75%`         | *Percent* of the total number of optional clauses are *necessary*. The number computed from the percentage is rounded down and used as the minimum.                                           |
    /// | `-25%`        | *Percent* of the total number of optional clauses can be *missing*. The number computed from the percentage is rounded down, before being subtracted from the total to determine the minimum. |
    /// | `3<90%`       | *e.g.*: if there are 1 to 3 clauses they are all required, but for 4 or more clauses only 90% are required.                                                                                   |
    /// | `2<-25% 9<-3` | *e.g.*: if there are 1 or 2 clauses both are required, if there are 3-9 clauses all but 25% are required, and if there are more than 9 clauses, all but three are required.                   |
    ///
    /// [Controls]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

#[cfg(feature = "graphql")]
impl BooleanQueryInput {
    /// Returns `true` if this `BooleanQueryInput` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.must.is_empty()
            && self.filter.is_empty()
            && self.should.is_empty()
            && self.must_not.is_empty()
    }

    /// Appends a `filter` to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<QueryInput>) {
        // TODO: should we always default to `filter` context?
        self.filter.push(filter.into())
    }
}

#[cfg(feature = "graphql")]
impl<T: Into<QueryInput>> From<T> for BooleanQueryInput {
    #[inline]
    fn from(filter: T) -> BooleanQueryInput {
        BooleanQueryInput {
            must: vec![],
            filter: vec![filter.into()],
            should: vec![],
            must_not: vec![],
            minimum_should_match: None,
            boost: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<BooleanQuery> for BooleanQueryInput {
    #[inline]
    fn from(query: BooleanQuery) -> Self {
        Self {
            must: query.must.into_iter().map(Into::into).collect(),
            filter: query.filter.into_iter().map(Into::into).collect(),
            should: query.should.into_iter().map(Into::into).collect(),
            must_not: query.must_not.into_iter().map(Into::into).collect(),
            minimum_should_match: query.minimum_should_match.map(Into::into),
            boost: query.boost.map(Into::into),
        }
    }
}

/// A [query] that matches documents matching boolean combinations of other
/// queries. It is built using one or more boolean clauses, each clause with a
/// typed occurrence.
///
/// [query]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "BooleanFilter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct BooleanQuery {
    /// The clause (query) **must** appear in matching documents and *will
    /// contribute to the score* of this query.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must: Vec<Query>,

    /// The clause (query) **must** appear in matching documents. However unlike
    /// `must`, the *score of the query will be ignored*. Query clauses are
    /// executed in [filter context], meaning that scoring is ignored and
    /// clauses are considered for caching.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<Query>,

    /// The clause (query) **should** appear in the matching document.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub should: Vec<Query>,

    /// The clause (query) **must not** appear in the matching documents. Clauses
    /// are executed in [filter context] meaning that *scoring is ignored* and
    /// clauses are considered for caching. Because scoring is ignored, a score
    /// of 0 for all documents is returned.
    ///
    /// [filter context]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_not: Vec<Query>,

    /// [Controls] how many optional (`should`) parameters must match.
    ///
    /// | Example       | Description                                                                                                                                                                                   |
    /// |---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `3`           | *Fixed value* regardless of the number of optional clauses                                                                                                                                    |
    /// | `-2`          | Total number of optional clauses, *minus* this number should be mandatory                                                                                                                     |
    /// | `75%`         | *Percent* of the total number of optional clauses are *necessary*. The number computed from the percentage is rounded down and used as the minimum.                                           |
    /// | `-25%`        | *Percent* of the total number of optional clauses can be *missing*. The number computed from the percentage is rounded down, before being subtracted from the total to determine the minimum. |
    /// | `3<90%`       | *e.g.*: if there are 1 to 3 clauses they are all required, but for 4 or more clauses only 90% are required.                                                                                   |
    /// | `2<-25% 9<-3` | *e.g.*: if there are 1 or 2 clauses both are required, if there are 3-9 clauses all but 25% are required, and if there are more than 9 clauses, all but three are required.                   |
    ///
    /// [Controls]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,

    /// Floating point number used to decrease or increase the
    /// [relevance scores] of a query. (Defaults to `1.0`.)
    ///
    /// You can use the boost parameter to adjust relevance scores for searches
    /// containing two or more queries.
    ///
    /// Boost values are relative to the default value of `1.0`. A boost value
    /// between `0` and `1.0` decreases the relevance score. A value greater
    /// than  `1.0` increases the relevance score.
    ///
    /// [relevance scores]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl BooleanQuery {
    /// Returns `true` if this `BooleanQuery` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.must.is_empty()
            && self.filter.is_empty()
            && self.should.is_empty()
            && self.must_not.is_empty()
    }

    /// Appends a `filter` to the current list of filters.
    #[inline]
    pub fn push(&mut self, filter: impl Into<Query>) {
        // TODO: should we always default to `filter` context?
        self.filter.push(filter.into())
    }
}

#[cfg(feature = "graphql")]
impl From<BooleanQueryInput> for BooleanQuery {
    #[inline]
    fn from(input: BooleanQueryInput) -> BooleanQuery {
        // TODO: why isn't the blanket impl in the std library auto impl these?
        BooleanQuery {
            must: input.must.into_iter().map(Into::into).collect(),
            filter: input.filter.into_iter().map(Into::into).collect(),
            should: input.should.into_iter().map(Into::into).collect(),
            must_not: input.must_not.into_iter().map(Into::into).collect(),
            minimum_should_match: input.minimum_should_match.map(Into::into),
            boost: input.boost.map(Into::into),
        }
    }
}

impl<T: Into<Query>> From<T> for BooleanQuery {
    #[inline]
    fn from(filter: T) -> BooleanQuery {
        BooleanQuery {
            must: vec![],
            filter: vec![filter.into()],
            should: vec![],
            must_not: vec![],
            minimum_should_match: None,
            boost: None,
        }
    }
}

/// A single query to perform for this search request.
///
/// **Note**: If a filter over a list of objects does not return the
/// expected results, try a `NestedQueryInput`.
///
/// **Note**: Specifying more than one field will result in an error.
///
/// **TODO**: Change this type once [union input types] are supported by GraphQL
/// to only allow specifying a single field.
///
/// [union input types]: https://github.com/graphql/graphql-spec/blob/master/rfcs/InputUnion.md
#[allow(missing_docs)]
#[cfg(feature = "graphql")]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(async_graphql::InputObject, Serialize, Clone, Debug)]
#[graphql(name = "FilterInput")]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct QueryInput {
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<ExistsQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<TermQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<TermsQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<PrefixQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<RegexpQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_: Option<MatchQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simple_query_string: Option<SimpleQueryStringQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<QueryStringQueryInput>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<NestedQueryInput>,

    /// A nested bool query.
    #[serde(rename = "bool", default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanQueryInput>,
}

#[cfg(feature = "graphql")]
impl From<Query> for QueryInput {
    #[inline]
    fn from(query: Query) -> Self {
        Self {
            exists: query.exists.map(Into::into),
            term: query.term.map(Into::into),
            terms: query.terms.map(Into::into),
            range: query.range.map(Into::into),
            prefix: query.prefix.map(Into::into),
            regexp: query.regexp.map(Into::into),
            match_: query.match_.map(Into::into),
            simple_query_string: query.simple_query_string.map(Into::into),
            query_string: query.query_string.map(Into::into),
            nested: query.nested.map(Into::into),
            boolean: query.boolean.map(Into::into),
        }
    }
}

#[cfg(feature = "graphql")]
impl From<ExistsQueryInput> for QueryInput {
    #[inline]
    fn from(filter: ExistsQueryInput) -> QueryInput {
        QueryInput {
            exists: Some(filter),
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<TermQueryInput> for QueryInput {
    #[inline]
    fn from(filter: TermQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: Some(filter),
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<TermsQueryInput> for QueryInput {
    #[inline]
    fn from(filter: TermsQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: Some(filter),
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<RangeQueryInput> for QueryInput {
    #[inline]
    fn from(filter: RangeQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: Some(filter),
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<PrefixQueryInput> for QueryInput {
    #[inline]
    fn from(filter: PrefixQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: Some(filter),
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<RegexpQueryInput> for QueryInput {
    #[inline]
    fn from(filter: RegexpQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: Some(filter),
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<MatchQueryInput> for QueryInput {
    #[inline]
    fn from(filter: MatchQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: Some(filter),
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<SimpleQueryStringQueryInput> for QueryInput {
    #[inline]
    fn from(filter: SimpleQueryStringQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: Some(filter),
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<QueryStringQueryInput> for QueryInput {
    #[inline]
    fn from(filter: QueryStringQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: Some(filter),
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<NestedQueryInput> for QueryInput {
    #[inline]
    fn from(filter: NestedQueryInput) -> QueryInput {
        QueryInput {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: Some(filter),
            boolean: None,
        }
    }
}

/// A single search query.
///
/// **Note**: This should *never* have more than *one* defined (and non-null) field.
#[allow(missing_docs)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "Filter"))]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", builder(field_defaults(setter(into))))]
pub struct Query {
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<ExistsQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<TermQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<TermsQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<PrefixQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<RegexpQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_: Option<MatchQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simple_query_string: Option<SimpleQueryStringQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<QueryStringQuery>,

    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<NestedQuery>,

    /// A nested bool query.
    #[cfg_attr(feature = "builder", builder(default))]
    #[serde(rename = "bool", default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanQuery>,
}

#[cfg(feature = "graphql")]
impl From<QueryInput> for Query {
    #[inline]
    fn from(input: QueryInput) -> Query {
        Query {
            exists: input.exists.map(Into::into),
            term: input.term.map(Into::into),
            terms: input.terms.map(Into::into),
            range: input.range.map(Into::into),
            prefix: input.prefix.map(Into::into),
            regexp: input.regexp.map(Into::into),
            match_: input.match_.map(Into::into),
            simple_query_string: input.simple_query_string.map(Into::into),
            query_string: input.query_string.map(Into::into),
            nested: input.nested.map(Into::into),
            boolean: input.boolean.map(Into::into),
        }
    }
}

impl From<ExistsQuery> for Query {
    #[inline]
    fn from(filter: ExistsQuery) -> Query {
        Query {
            exists: Some(filter),
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<TermQuery> for Query {
    #[inline]
    fn from(filter: TermQuery) -> Query {
        Query {
            exists: None,
            term: Some(filter),
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<TermsQuery> for Query {
    #[inline]
    fn from(filter: TermsQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: Some(filter),
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<RangeQuery> for Query {
    #[inline]
    fn from(filter: RangeQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: Some(filter),
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

#[cfg(feature = "graphql")]
impl From<PrefixQuery> for Query {
    #[inline]
    fn from(filter: PrefixQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: Some(filter),
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<RegexpQuery> for Query {
    #[inline]
    fn from(filter: RegexpQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: Some(filter),
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<MatchQuery> for Query {
    #[inline]
    fn from(filter: MatchQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,

            match_: Some(filter),
            simple_query_string: None,
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<SimpleQueryStringQuery> for Query {
    #[inline]
    fn from(filter: SimpleQueryStringQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: Some(filter),
            query_string: None,
            nested: None,
            boolean: None,
        }
    }
}

impl From<QueryStringQuery> for Query {
    #[inline]
    fn from(filter: QueryStringQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: Some(filter),
            nested: None,
            boolean: None,
        }
    }
}

impl From<NestedQuery> for Query {
    #[inline]
    fn from(filter: NestedQuery) -> Query {
        Query {
            exists: None,
            term: None,
            terms: None,
            range: None,
            prefix: None,
            regexp: None,
            match_: None,
            simple_query_string: None,
            query_string: None,
            nested: Some(filter),
            boolean: None,
        }
    }
}

/// Describes a field that can be queried and its type.
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "FilterField"))]
#[derive(Debug)]
pub struct QueryField {
    /// The field name.
    pub field: String,

    // TODO: rename to `ty` once https://github.com/async-graphql/async-graphql/issues/164
    /// The type
    pub type_: String,
}

impl QueryField {
    /// Create a new `QueryField`.
    #[inline]
    pub fn new(field: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            type_: ty.into(),
        }
    }
}
