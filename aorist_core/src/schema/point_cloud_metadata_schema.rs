use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::derived_asset_schema::DerivedAssetSchema;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
use aorist_primitives::attribute;
use aorist_attributes::*;

#[aorist]
pub struct PointCloudMetadataSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub source: AoristRef<StaticDataTable>,
}
impl PointCloudMetadataSchema {
		pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        vec![
            attribute! { FreeText(
                "prefix".to_string(), 
                Some("File Prefix".to_string()), 
                false
            )},
            attribute! { FreeText(
                "comp_spatialreference".to_string(), 
                None,
                false
            )}, 
            attribute! { Boolean(
                "compressed".to_string(),
                Some("Object is compressed".to_string()),
                false
            )},
            attribute! { Count(
                "count".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "creation_doy".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "creation_year".to_string(),
                None,
                false
            )},
            attribute! { Int64Identifier(
                "dataformat_id".to_string(), 
                None,
                false
            )}, 
            attribute! { FreeText(
                "dataoffset".to_string(), 
                None,
                false
            )}, 
            attribute! { FreeText(
                "gtiff".to_string(), 
                None,
                false
            )}, 
            attribute! { NaturalNumber(
                "header_size".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "major_version".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "maxx".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "maxy".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "maxz".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "minor_version".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "minx".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "miny".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "minz".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "offset_x".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "offset_y".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "offset_z".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "scale_x".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "scale_y".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "scale_z".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "point_length".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "maxy".to_string(),
                None,
                false
            )},
            attribute! { NaturalNumber(
                "maxz".to_string(),
                None,
                false
            )},
            attribute! { Int64Identifier(
                "project_id".to_string(), 
                None,
                false
            )}, 
            attribute! { Int64Identifier(
                "software_id".to_string(), 
                None,
                false
            )}, 
            attribute! { Int64Identifier(
                "system_id".to_string(), 
                None,
                false
            )}, 
            attribute! { FreeText(
                "spatialreference".to_string(), 
                None,
                false
            )}, 
            attribute! { FreeText(
                "srs".to_string(), 
                None,
                false
            )}, 
            attribute! { FreeText(
                "vlrs".to_string(), 
                Some("JSON for metadata vlrs".to_string()), 
                false
            )}, 
        ]
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
impl DerivedAssetSchema<'_> for PointCloudMetadataSchema {
    type SourceAssetType = StaticDataTable; 
    fn get_source(&self) -> AoristRef<Self::SourceAssetType> {
        self.source.clone()
    }
}
