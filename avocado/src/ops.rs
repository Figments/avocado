//! High-level database operations: query, update, delete, etc.

use std::fmt::Debug;
use serde::Deserialize;
use bson::{ Bson, Document };
use mongodb::options::{
    FindOptions,
    CountOptions,
    WriteConcern,
    DistinctOptions,
    AggregateOptions,
    FindOneAndUpdateOptions,
};
use crate::{
    doc::Doc,
    error::Result,
};

/// A counting-only query.
pub trait Count<T: Doc>: Debug {
    /// Filter for this query. Defaults to an empty filter,
    /// yielding the number of *all* documents in the collection.
    fn filter(&self) -> Document {
        Default::default()
    }

    /// Options for this query.
    fn options(&self) -> CountOptions {
        T::count_options()
    }
}

/// A query for returning the distinct values of a field.
pub trait Distinct<T: Doc>: Debug {
    /// The type of the field of which the distinct values will be returned.
    type Output: for<'a> Deserialize<'a>;

    /// The name of the field of which the distinct values will be returned.
    const FIELD: &'static str;

    /// Optional filter restricting which values are taken into account.
    /// Defaults to no filtering.
    fn filter(&self) -> Document {
        Default::default()
    }

    /// Optional transform applied to each returned raw BSON. Can be used to
    /// adjust the structure of the loosely-typed data so that it fits
    /// what is expected by `<Self::Output as Deserialize>::deserialize()`.
    ///
    /// The default implementation just returns its argument verbatim.
    fn transform(raw: Bson) -> Result<Bson> {
        Ok(raw)
    }

    /// Options for this query.
    fn options(&self) -> DistinctOptions {
        T::distinct_options()
    }
}

/// An aggregation pipeline.
pub trait Pipeline<T: Doc>: Debug {
    /// The type of the values obtained by running this pipeline.
    type Output: for<'a> Deserialize<'a>;

    /// The stages of the aggregation pipeline.
    fn stages(&self) -> Vec<Document>;

    /// Optional transform applied to each returned raw document. Can be used
    /// to adjust the structure of the loosely-typed data so that it fits
    /// what is expected by `<Self::Output as Deserialize>::deserialize()`.
    ///
    /// The default implementation just returns its argument verbatim.
    fn transform(raw: Document) -> Result<Bson> {
        Ok(raw.into())
    }

    /// Options for this pipeline.
    fn options(&self) -> AggregateOptions {
        T::aggregate_options()
    }
}

/// A regular query (`find_one()` or `find_many()`) operation.
pub trait Query<T: Doc>: Debug {
    /// The type of the results obtained by executing the query. Often it's just
    /// the document type, `T`. TODO(H2CO3): make it default to `T` (#29661).
    type Output: for<'a> Deserialize<'a>;

    /// Filter for restricting returned values. Defaults to an empty filter,
    /// resulting in *all* documents of the collection being returned.
    fn filter(&self) -> Document {
        Default::default()
    }

    /// Optional transform applied to each returned raw document. Can be used
    /// to adjust the structure of the loosely-typed data so that it fits
    /// what is expected by `<Self::Output as Deserialize>::deserialize()`.
    ///
    /// The default implementation just returns its argument verbatim.
    fn transform(raw: Document) -> Result<Bson> {
        Ok(raw.into())
    }

    /// Options for this query.
    fn options(&self) -> FindOptions {
        T::query_options()
    }
}

/// An update (but not an upsert) operation.
pub trait Update<T: Doc>: Debug {
    /// Filter for restricting documents to update.
    fn filter(&self) -> Document;

    /// The update to perform on matching documents.
    fn update(&self) -> Document;

    /// Options for this update operation.
    fn options(&self) -> WriteConcern {
        T::update_options()
    }
}

/// An upsert (update or insert) operation.
pub trait Upsert<T: Doc>: Debug {
    /// Filter for restricting documents to upsert.
    fn filter(&self) -> Document;

    /// The upsert to perform on matching documents.
    fn upsert(&self) -> Document;

    /// Options for this upsert operation.
    fn options(&self) -> WriteConcern {
        T::upsert_options()
    }
}

/// A deletion / removal operation.
pub trait Delete<T: Doc>: Debug {
    /// Filter for restricting documents to delete.
    fn filter(&self) -> Document;

    /// Writing options for this deletion operation.
    fn options(&self) -> WriteConcern {
        T::delete_options()
    }
}

/// An operation for querying and updating the same document atomically,
/// in a single step.
pub trait FindAndUpdate<T: Doc>: Debug {
    /// The type of the results returned by the operation. Often it's just
    /// the document type, `T`. TODO(H2CO3): make it default to `T` (#29661).
    type Output: for<'a> Deserialize<'a>;

    /// Filter for restricting documents to update or upsert.
    fn filter(&self) -> Document;

    /// The update or upsert to perform.
    fn update(&self) -> Document;

    /// Optional transform applied to the returned raw document. Can be used
    /// to adjust the structure of the loosely-typed data so that it fits
    /// what is expected by `<Self::Output as Deserialize>::deserialize()`.
    ///
    /// The default implementation just returns its argument verbatim.
    fn transform(raw: Document) -> Result<Bson> {
        Ok(raw.into())
    }

    /// Options for this query-and-update operation.
    fn options(&self) -> FindOneAndUpdateOptions {
        T::find_and_update_options()
    }
}

/////////////////////////////////////////////
// Blanket and convenience implementations //
/////////////////////////////////////////////

impl<T: Doc> Count<T> for Document {
    fn filter(&self) -> Document {
        self.clone()
    }
}

impl<T: Doc> Query<T> for Document {
    type Output = T;

    fn filter(&self) -> Document {
        self.clone()
    }
}

impl<T: Doc> Delete<T> for Document {
    fn filter(&self) -> Document {
        self.clone()
    }
}

impl<T: Doc, Q: Count<T>> Count<T> for &Q {
    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn options(&self) -> CountOptions {
        (**self).options()
    }
}

impl<T: Doc, Q: Distinct<T>> Distinct<T> for &Q {
    type Output = Q::Output;

    const FIELD: &'static str = Q::FIELD;

    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn transform(bson: Bson) -> Result<Bson> {
        Q::transform(bson)
    }

    fn options(&self) -> DistinctOptions {
        (**self).options()
    }
}

impl<T: Doc, P: Pipeline<T>> Pipeline<T> for &P {
    type Output = P::Output;

    fn stages(&self) -> Vec<Document> {
        (**self).stages()
    }

    fn transform(doc: Document) -> Result<Bson> {
        P::transform(doc)
    }

    fn options(&self) -> AggregateOptions {
        (**self).options()
    }
}

impl<T: Doc, Q: Query<T>> Query<T> for &Q {
    type Output = Q::Output;

    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn transform(doc: Document) -> Result<Bson> {
        Q::transform(doc)
    }

    fn options(&self) -> FindOptions {
        (**self).options()
    }
}

impl<T: Doc, U: Update<T>> Update<T> for &U {
    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn update(&self) -> Document {
        (**self).update()
    }

    fn options(&self) -> WriteConcern {
        (**self).options()
    }
}

impl<T: Doc, U: Upsert<T>> Upsert<T> for &U {
    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn upsert(&self) -> Document {
        (**self).upsert()
    }

    fn options(&self) -> WriteConcern {
        (**self).options()
    }
}

impl<T: Doc, Q: Delete<T>> Delete<T> for &Q {
    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn options(&self) -> WriteConcern {
        (**self).options()
    }
}

impl<T: Doc, U: FindAndUpdate<T>> FindAndUpdate<T> for &U {
    type Output = U::Output;

    fn filter(&self) -> Document {
        (**self).filter()
    }

    fn update(&self) -> Document {
        (**self).update()
    }

    fn transform(raw: Document) -> Result<Bson> {
        U::transform(raw)
    }

    fn options(&self) -> FindOneAndUpdateOptions {
        (**self).options()
    }
}
