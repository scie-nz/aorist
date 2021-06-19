#[cfg(feature = "sql")]
use aorist_attributes::TSQLAttribute;
use aorist_attributes::{
    AttributeOrTransform, Count, Regressor, TAttribute, TBigQueryAttribute, TOrcAttribute,
    TPostgresAttribute, TPrestoAttribute, TSQLiteAttribute,
};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
include!(concat!(env!("OUT_DIR"), "/attributes.rs"));
