/* Following prescribed Record of Processing Activity by cnil.fr.
See: https://www.cnil.fr/en/record-processing-activities */
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::{AoristConcept, AoristConceptChildren, ConceptEnum};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use aorist_primitives::gdpr_data_type;
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub struct GDPRStakeholder {
    name: String,
    street_address: String,
    postcode: String,
    city: String,
    country: String,
    phone_number: String,
    email: String,
    external_organization_name: Option<String>,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub struct GDPRDataProcessingPurpose {
    main_purpose: String,
    sub_purposes: Option<Vec<String>>,
}

gdpr_data_type! {PersonalIdentificationData}

gdpr_data_type! {PersonalLifeData}

gdpr_data_type! {EconomicSituationData}

gdpr_data_type! {TelecommunicationsData}

gdpr_data_type! {LocationData}

gdpr_data_type! {OfficialPersonalIdentifier}

gdpr_data_type! {RacialOrEthnicOrigin}

gdpr_data_type! {PoliticalOpinions}

gdpr_data_type! {ReligionOrPhilosophicalBeliefs}

gdpr_data_type! {TradeUnionMembership}

gdpr_data_type! {GeneticData}

gdpr_data_type! {UniquelyIdentifyingBiometricData}

gdpr_data_type! {HealthData}

gdpr_data_type! {SexLifeAndOrientationData}

gdpr_data_type! {CriminalConvictionAndOffenceData}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub enum GDPRPersonalDataCategory {
    PersonalIdentificationData(PersonalIdentificationData),
    PersonalLifeData(PersonalLifeData),
    EconomicSituationData(EconomicSituationData),
    TelecommunicationsData(TelecommunicationsData),
    LocationData(LocationData),
    OfficialPersonalIdentifier(OfficialPersonalIdentifier),
    RacialOrEthnicOrigin(RacialOrEthnicOrigin),
    PoliticalOpinions(PoliticalOpinions),
    ReligionOrPhilosophicalBeliefs(ReligionOrPhilosophicalBeliefs),
    TradeUnionMembership(TradeUnionMembership),
    GeneticData(GeneticData),
    UniquelyIdentifyingBiometricData(UniquelyIdentifyingBiometricData),
    HealthData(HealthData),
    SexLifeAndOrientationData(SexLifeAndOrientationData),
    CriminalConvictionAndOffenceData(CriminalConvictionAndOffenceData),
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub enum GDPRDataSubjectCategory {
    Employees(String),
    InternalServices(String),
    Customers(String),
    Suppliers(String),
    ServiceProviders(String),
    PotentialCustomers(String),
    Applicants(String),
    Other(String),
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub enum GDPRDataProcessingRecipient {
    InternalDepartment(String),
    Processor(String),
    RecipientInThirdCountryOrInternationalOrganization(String),
    InstitutionalOrCommercialPartner(String),
    Other(String),
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub struct GDPRProcessorRecord {
    unique_short_name: String,
    name_of_processing_operation: String,
    processing_start_date: String,
    controller: GDPRStakeholder,
    data_protection_officer: GDPRStakeholder,
    representative: Option<GDPRStakeholder>,
    joint_controllers: Option<Vec<GDPRStakeholder>>,
    data_processing_purposes: Vec<GDPRDataProcessingPurpose>,
    personal_data_categories_used: Vec<GDPRPersonalDataCategory>,
    data_subject_categories: Vec<GDPRDataSubjectCategory>,
    data_processing_recipients: Vec<GDPRDataProcessingRecipient>,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub struct GDPRSecurityMeasuresStatement {
    traceability: Vec<String>,
    software_protection: Vec<String>,
    data_backup: Vec<String>,
    data_encryption: Vec<String>,
    user_access_control: Vec<String>,
    control_of_processors: Vec<String>,
    other: Vec<String>,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub enum GDPRDataTransferGuarantee {
    StandardContractualClauses(String),
    BindingCorporateRules(String),
    CountryProvidingAdequateLevelOfProtection(String),
    PrivacyShield(String),
    CodeOfConduct(String),
    Certification(String),
    DerogationsPerArticle49GDPR(String),
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize, FromPyObject)]
pub struct GDPRThirdPartyCountryOrInternationalOrganizationTransferRecord {
    recipient_organization_name: String,
    iso_3166_2c_country_code: String,
    data_transfer_guarantees: Vec<GDPRDataTransferGuarantee>,
    links_to_relevant_documents: Option<Vec<String>>,
}

#[aorist_concept]
pub struct ComplianceConfig {
    description: String,
    data_about_human_subjects: bool,
    contains_personally_identifiable_information: bool,
    gdpr_processor_record: Option<GDPRProcessorRecord>,
}
