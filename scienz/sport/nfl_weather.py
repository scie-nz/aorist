from aorist import (
    Attribute,
    NaturalNumber,
    StringIdentifier,
    DateString,
    Year,
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
)

attributes = [
    Attribute(StringIdentifier("id")),
    Attribute(StringIdentifier("home_team")),
    Attribute(NaturalNumber("home_score")),
    Attribute(StringIdentifier("away_team")),
    Attribute(NaturalNumber("away_score")),
    Attribute(NaturalNumber("temperature")),
    Attribute(NaturalNumber("wind_chill")),
    Attribute(StringIdentifier("humidity")),
    Attribute(NaturalNumber("wind_mph")),
    Attribute(StringIdentifier("weather")),
    Attribute(DateString("date")),
]

nfl_weather_datum = RowStruct(
    name="nfl_weather_datum",
    attributes=attributes,
)

nfl_weather_schema = default_tabular_schema(
    DatumTemplate(nfl_weather_datum), attributes
)

table = Asset(StaticDataTable(
            name="nfl_weather_table",
            schema=DataSchema(nfl_weather_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("http://nflsavant.com/dump/weather_20131231.csv"),
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
            tag="nfl_weather",
            ))

nfl_weather_dataset = DataSet(
    name="nfl_weather_dataset",
    description="""
        Data about the Weather for NFL games 1960-2013. From NFLsavant.com.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(nfl_weather_datum)],
    assets={
        "nfl_weather_data": table,
    },
    access_policies=[]
)
