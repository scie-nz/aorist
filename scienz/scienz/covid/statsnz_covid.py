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
    Year,
    CountryName,
)

attributes = [
    Attribute(StringIdentifier("Direction")),
    Attribute(Year("Year")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("Weekday")),
    Attribute(CountryName("Country")),
    Attribute(StringIdentifier("Comodity")),
    Attribute(StringIdentifier("Transport_Mode")),
    Attribute(StringIdentifier("Measure")),
    Attribute(NaturalNumber("Value")),
    Attribute(NaturalNumber("Cumulative")),
]

statsnz_covid_datum = RowStruct(
    name="statsnz_covid_datum",
    attributes=attributes,
)

statsnz_covid_schema = default_tabular_schema(
    DatumTemplate(statsnz_covid_datum), attributes
)

table = Asset(StaticDataTable(
            name="stats_nz_table",
            schema=DataSchema(statsnz_covid_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://www.stats.govt.nz/assets/Uploads/Effects-of-"
                                     "COVID-19-on-trade/Effects-of-COVID-19-on-trade-At-"
                                     "14-April-2021-provisional/Download-data/effects-of"
                                     "-covid-19-on-trade-at-14-April-2021-provisional.csv"),
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
            tag="statsnz_covid",
            ))

statsnz_covid_dataset = DataSet(
    name="statsnz_covid",
    description="""
        New Zealand's daily goods trade with the world. Comparing 
         values with previous years shows the potential impacts of COVID-19.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(statsnz_covid_datum)],
    assets={
        "StatsNZ_effects_of_COVID-19_on_trade_data": table,
    },
    access_policies=[]
)
