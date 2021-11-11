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
    FloatLatitude,
)

attributes = [
    Attribute(StringIdentifier("country")),
    Attribute(StringIdentifier("country_code")),
    Attribute(StringIdentifier("continent")),
    Attribute(NaturalNumber("population")),
    Attribute(StringIdentifier("indicator")),
    Attribute(NaturalNumber("weekly_count")),
    Attribute(DateString("year_week")),
    Attribute(FloatLatitude("rate_14_day")),
    Attribute(NaturalNumber("cumulative_count")),
    Attribute(FreeText("source")),
]

covid_euro_datum = RowStruct(
    name="covid_euro_datum",
    attributes=attributes,
)

covid_euro_schema = default_tabular_schema(
    DatumTemplate(covid_euro_datum), attributes
)

table = Asset(StaticDataTable(
            name="covid_euro_table",
            schema=DataSchema(covid_euro_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://opendata.ecdc.europa.eu/covid19/nationalcasedeath/csv"),
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
            tag="ECDC_covid",
            ))

ECDC_covid_dataset = DataSet(
    name="covid_euro",
    description="""
        Contains information on the 14-day notification rate of
         newly reported COVID-19 cases per 100 000 population
         and the 14-day notification rate of reported deaths per
         million populatoin by week and country.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(covid_euro_datum)],
    assets={
        "ECDC_COVID-19_data": table,
    },
    access_policies=[]
)
