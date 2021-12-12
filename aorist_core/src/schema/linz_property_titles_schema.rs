use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, primary_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

// Datasets 50804 (non-PII) and 50805 (PII)
primary_schema! {
    name: LINZPropertyTitlesSchema,
    attributes:
      event_type: Factor("INSERT/DELETE/UPDATE", false),
      event_timestamp: ISO8601Timestamp("When the event was published (weekly)", false),
      changeset_id: KeyStringIdentifier("TBD, is distinct within a given event_timestamp, but not across timestamps", false),
      geometry: Geometry("Polygon describing the title, if any", true),
      id: Int64Identifier("TBD, is distinct within a given event_timestamp but not across timestamps", false), // TODO max value in practice is 4M - smaller int type?
      title_no: FreeText("May align with entry in LINZPrimaryParcelsSchema.attributes.titles[]?", false),
      status: Factor("PRTC or LIVE, but others like UNCV may be possible", false),
      type: Factor("Type of title e.g. 'Freehold'", false),
      land_district: Factor("Which of the 12 historic LINZ Land Districts the parcel falls under", false),
      issue_date: ISO8601Timestamp("When the title was issued", false),
      guarantee_status: Factor("'Limited as to Parcels' or 'Guarantee'", false),
      estate_description: FreeText("Inconsistent/arbitrary text like 'Fee Simple, 1/2, Lot 24 DP 123, 123m2'", true),
      number_owners: Int64("Owner count, may be zero, in non-PII dataset (50804) only", true), // TODO max value in practice is ~5000 - smaller int type than int64?
      owners: FreeText("Owner full names, In PII dataset (50805) only", true), // TODO declare as nullable array of strings, TODO should we create a separate LINZPropertyTitlesPIISchema for this? (only one column difference)
      spatial_extents_shared: Boolean("TBD", false)
}
