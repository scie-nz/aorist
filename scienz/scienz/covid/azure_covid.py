from aorist import (
    Attribute,
    NaturalNumber,
    StringIdentifier,
    DateString,
    POSIXTimestamp,
    PositiveFloat,
    default_tabular_schema,
    RowStruct,
    StaticDataTable,
    DataSchema,
    StorageSetup,
    RemoteStorageSetup,
    Storage,
    RemoteStorage,
    RemoteLocation,
    CSVEncoding,
    Encoding,
    DataSet,
    DatumTemplate,
    Asset,
    WebLocation,
    FileBasedStorageLayout,
    CSVHeader,
    FileHeader,
    APIOrFileLayout,
    SingleFileLayout,
    FreeText,
    Empty,
    FIPSStateCode,
    IntegerNumber,
)

attributes = [
    Attribute(DateString("date")),
    Attribute(StringIdentifier("state")),
    Attribute(NaturalNumber("positive")),
    Attribute(NaturalNumber("negative")),
    Attribute(NaturalNumber("pending")),
    Attribute(NaturalNumber("hospitalized_currently")),
    Attribute(NaturalNumber("hospitalized_cumulative")),
    Attribute(NaturalNumber("in_icu_currently")),
    Attribute(NaturalNumber("in_icu_cumulative")),
    Attribute(NaturalNumber("on_ventilator_currently")),
    Attribute(NaturalNumber("on_ventilator_cumultive")),
    Attribute(NaturalNumber("recovered")),
    Attribute(Empty("data_quality_grade")),
    Attribute(DateString("last_update_et")),
    Attribute(FreeText("hash")),
    Attribute(DateString("date_checked")),
    Attribute(NaturalNumber("death")),
    Attribute(NaturalNumber("hospitalized")),
    Attribute(NaturalNumber("total")),
    Attribute(NaturalNumber("total_test_results")),
    Attribute(NaturalNumber("pos_neg")),
    Attribute(FIPSStateCode("fips")),
    Attribute(IntegerNumber("death_increase")),
    Attribute(IntegerNumber("hospitalized_increase")),
    Attribute(IntegerNumber("negative_increase")),
    Attribute(IntegerNumber("positive_increase")),
    Attribute(IntegerNumber("total_test_results_increase")),
    Attribute(FIPSStateCode("fips_code")),
    Attribute(StringIdentifier("iso_subdivision")),
    Attribute(DateString("load_time")),
    Attribute(StringIdentifier("iso_country")),
]

microsoft_covid_datum = RowStruct(
    name="microsoft_covid_datum",
    attributes=attributes,
)

microsoft_covid_schema = default_tabular_schema(
    DatumTemplate(microsoft_covid_datum), attributes
)

table = Asset(StaticDataTable(
            name="microsoft_covid_table",
            schema=DataSchema(microsoft_covid_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://pandemicdatalake.blob.core.windows.net/public/"
                                     "curated/covid-19/covid_tracking/latest/covid_tracking.csv"),
                        )
                    ),
                    layout=APIOrFileLayout(
                        FileBasedStorageLayout(
                            SingleFileLayout()
                        ),
                    ),
                    encoding=Encoding(CSVEncoding(header=FileHeader(
                        CSVHeader(num_lines=1)
                    ))),
                )),
            )),
            tag="azure_covid_microsoft",
            ))

azure_covid_dataset = DataSet(
    name="azure-covid",
    description="""
        Statistics about COVID 19 in the United States compiled by Microsoft,
         including test, confirmed cases, hospitalizations, and patient
         outcomes from every US state and territory, up until 2021-03-07.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(microsoft_covid_datum)],
    assets={
       "Microsoft COVID-19 data": table,
    },
    access_policies=[]
)
