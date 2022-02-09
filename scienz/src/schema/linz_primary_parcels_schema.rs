use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, primary_schema};
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AoristRef;
use aorist_util::{AString, AUuid, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Dataset 50772
primary_schema! {
    name: LINZPrimaryParcelsSchema,
    attributes:
      event_type: Factor("INSERT/DELETE/UPDATE", false),
      event_timestamp: ISO8601Timestamp("When the event was published (weekly)", false),
      changeset_id: KeyStringIdentifier("TBD, is distinct within a given event_timestamp, but not across timestamps", false),
      geometry: Geometry("Polygon describing the parcel, if any", true),
      id: Int64Identifier("TBD, is distinct within a given event_timestamp but not across timestamps", false), // TODO max value in practice is 8M - smaller int type?
      appelation: FreeText("Arbitrary text like 'Lot 123 DP 321'", true),
      affected_surveys: FreeText("Surveys that relate to this parcel if any", true),
      parcel_intent: Factor("Type of parcel e.g. 'Fee Simple Title'", false),
      statutory_actions: FreeText("Arbitrary text describing status", true),
      land_district: Factor("Which of the 12 historic LINZ Land Districts the parcel falls under", false),
      titles: FreeText("Zero or more LINZPropertyTitles that this parcel is associated with", true), // TODO declare as nullable array of strings
      survey_area: Int64("Area (in m2?) from a survey of the parcel, if any", true),
      calc_area: Int64("Pre-calculated area (in m2?) based on the geometry", false)
}
