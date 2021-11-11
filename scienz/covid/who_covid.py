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
    CountryName,
    IntegerNumber,
)

attributes = [
    Attribute(DateString("Date_reported")),
    Attribute(StringIdentifier("Country_code")),
    Attribute(CountryName("Country")),
    Attribute(StringIdentifier("WHO_region")),
    Attribute(NaturalNumber("New_cases")),
    Attribute(NaturalNumber("Cumulative_cases")),
    Attribute(NaturalNumber("New_deaths")),
    Attribute(NaturalNumber("Cumulative_deaths")),
]

who_covid_datum = RowStruct(
    name="who_covid_datum",
    attributes=attributes,
)

who_covid_schema = default_tabular_schema(
    DatumTemplate(who_covid_datum), attributes
)

table = Asset(StaticDataTable(
            name="who_covid_table",
            schema=DataSchema(who_covid_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://covid19.who.int/WHO-COVID-19-global-data.csv"),
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
            tag="WHO_covid",
            ))

who_covid_dataset = DataSet(
    name="who_covid",
    description="""
        COVID-19 new and cumulative cases and deaths by country,
         from the World Health Organisation.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(who_covid_datum)],
    assets={
        "World_Health_Organisation_COVID-19_data": table,
    },
    access_policies=[]
)
