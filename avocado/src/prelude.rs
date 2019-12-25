//! The Avocado prelude provides re-exports of the most commonly used traits
//! and types for convenience, including ones from crates `bson` and `mongodb`.

pub use crate::{
    db::DatabaseExt,
    coll::{ Collection, InsertManyErrorContext },
    doc::Doc,
    uid::Uid,
    ops::*,
    ext::*,
    literal::{ IndexType, Order, BsonType },
    error::Error as AvocadoError,
    error::ErrorKind as AvocadoErrorKind,
    error::Result as AvocadoResult,
};
pub use bson::{ Bson, Document, oid::ObjectId, doc, bson };
pub use mongodb::{
    Client, Database,
    options::{
        IndexModel, FindOptions,
        FindOneAndUpdateOptions, ReturnDocument,
    },
};
