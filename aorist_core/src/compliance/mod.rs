use aorist_primitives::AVec;
/* Following prescribed Record of Processing Activity by cnil.fr.
See: https://www.cnil.fr/en/record-processing-activities */
use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GDPRStakeholder {
    name: AString,
    street_address: AString,
    postcode: AString,
    city: AString,
    country: AString,
    phone_number: AString,
    email: AString,
    external_organization_name: Option<AString>,
}

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GDPRDataProcessingPurpose {
    main_purpose: AString,
    sub_purposes: Option<AVec<AString>>,
}

#[macro_export]
macro_rules! gdpr_data_type {
    ($name:ident
     $(, $field: ident : $field_type: ty)*) => {
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        #[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
        pub struct $name {
            description: AString,
            $(
                $field: $field_type,
            )*
        }
     };
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

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum GDPRDataSubjectCategory {
    Employees(AString),
    InternalServices(AString),
    Customers(AString),
    Suppliers(AString),
    ServiceProviders(AString),
    PotentialCustomers(AString),
    Applicants(AString),
    Other(AString),
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum GDPRDataProcessingRecipient {
    InternalDepartment(AString),
    Processor(AString),
    RecipientInThirdCountryOrInternationalOrganization(AString),
    InstitutionalOrCommercialPartner(AString),
    Other(AString),
}

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GDPRProcessorRecord {
    unique_short_name: AString,
    name_of_processing_operation: AString,
    processing_start_date: AString,
    controller: GDPRStakeholder,
    data_protection_officer: GDPRStakeholder,
    representative: Option<GDPRStakeholder>,
    joint_controllers: Option<AVec<GDPRStakeholder>>,
    data_processing_purposes: AVec<GDPRDataProcessingPurpose>,
    personal_data_categories_used: AVec<GDPRPersonalDataCategory>,
    data_subject_categories: AVec<GDPRDataSubjectCategory>,
    data_processing_recipients: AVec<GDPRDataProcessingRecipient>,
}

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GDPRSecurityMeasuresStatement {
    traceability: AVec<AString>,
    software_protection: AVec<AString>,
    data_backup: AVec<AString>,
    data_encryption: AVec<AString>,
    user_access_control: AVec<AString>,
    control_of_processors: AVec<AString>,
    other: AVec<AString>,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum GDPRDataTransferGuarantee {
    StandardContractualClauses(AString),
    BindingCorporateRules(AString),
    CountryProvidingAdequateLevelOfProtection(AString),
    PrivacyShield(AString),
    CodeOfConduct(AString),
    Certification(AString),
    DerogationsPerArticle49GDPR(AString),
}

#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[derive(PartialEq, Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct GDPRThirdPartyCountryOrInternationalOrganizationTransferRecord {
    recipient_organization_name: AString,
    iso_3166_2c_country_code: AString,
    data_transfer_guarantees: AVec<GDPRDataTransferGuarantee>,
    links_to_relevant_documents: Option<AVec<AString>>,
}

#[aorist]
pub struct ComplianceConfig {
    description: AString,
    data_about_human_subjects: bool,
    contains_personally_identifiable_information: bool,
    gdpr_processor_record: Option<GDPRProcessorRecord>,
}
