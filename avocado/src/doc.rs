//! A document is a direct member of a collection.

use serde::{ Serialize, Deserialize };
use mongodb::{
    options::{
        IndexModel,
        FindOptions,
        CountOptions,
        WriteConcern,
        DistinctOptions,
        AggregateOptions,
        InsertManyOptions,
        FindOneAndUpdateOptions,
    },
};
use crate::uid::Uid;

/// Implemented by top-level (direct collection member) documents only.
/// These types always have an associated top-level name and an `_id` field.
pub trait Doc: Serialize + for<'a> Deserialize<'a> {
    /// The type of the unique IDs for the document. A good default choice
    /// is `ObjectId`. TODO(H2CO3): make it default to `ObjectId` (#29661).
    type Id: Eq + Serialize + for <'a> Deserialize<'a>;

    /// The name of the collection within the database.
    const NAME: &'static str;

    /// Get the unique ID of this document if it exists.
    fn id(&self) -> Option<&Uid<Self>>;

    /// Set or change the unique ID of this document.
    fn set_id(&mut self, id: Uid<Self>);

    /// Returns the specifications of the indexes created on the collection.
    /// If not provided, returns an empty vector, leading to the collection not
    /// bearing any user-defined indexes. (The `_id` field will still be
    /// indexed, though, as defined by MongoDB.)
    fn indexes() -> Vec<IndexModel> {
        Vec::new()
    }

    /// Options for a count-only query.
    fn count_options() -> CountOptions {
        Default::default()
    }

    /// Options for a `distinct` query.
    fn distinct_options() -> DistinctOptions {
        Default::default()
    }

    /// Aggregation pipeline options.
    fn aggregate_options() -> AggregateOptions {
        Default::default()
    }

    /// Options for a regular query.
    fn query_options() -> FindOptions {
        Default::default()
    }

    /// Options for single and batch insertions.
    fn insert_options() -> InsertManyOptions {
        Default::default()
    }

    /// Options for a delete operation.
    fn delete_options() -> WriteConcern {
        Default::default()
    }

    /// Options for a (strictly non-upsert) update operation.
    fn update_options() -> WriteConcern {
        Default::default()
    }

    /// Options for upserting.
    fn upsert_options() -> WriteConcern {
        Default::default()
    }

    /// Options for find-and-update operations.
    fn find_and_update_options() -> FindOneAndUpdateOptions {
        Default::default()
    }
}
