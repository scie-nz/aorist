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
    Attribute(StringIdentifier("Country")),
    Attribute(StringIdentifier("League")),
    Attribute(Year("Season")),
    Attribute(DateString("Date")),
    Attribute(POSIXTimestamp("Time")),
    Attribute(StringIdentifier("Home")),
    Attribute(StringIdentifier("Away")),
    Attribute(NaturalNumber("HG")),
    Attribute(NaturalNumber("AG")),
    Attribute(StringIdentifier("Res")),
    Attribute(PositiveFloat("PH")),
    Attribute(PositiveFloat("PD")),
    Attribute(PositiveFloat("PA")),
    Attribute(PositiveFloat("MaxH")),
    Attribute(PositiveFloat("MaxD")),
    Attribute(PositiveFloat("MaxA")),
    Attribute(PositiveFloat("AvgH")),
    Attribute(PositiveFloat("AvgD")),
    Attribute(PositiveFloat("AvgA")),
]

mls_usa_datum = RowStruct(
    name="mls_usa_datum",
    attributes=attributes,
)
mls_usa_schema = default_tabular_schema(
    DatumTemplate(mls_usa_datum), attributes
)

table = Asset(StaticDataTable(
            name="mls_usa_table",
            schema=DataSchema(mls_usa_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://www.football-data.co.uk/new/USA.csv"),
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
            tag="major_league_soccer",
            ))

mls_usa_dataset = DataSet(
    name="mls_usa_dataset",
    description="""
        This dataset contains match statistics for Major League
         Soccer in the USA from 2012 onwards.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(mls_usa_datum)],
    assets={
        "Major_League_Soccer_data": table,
    },
    access_policies=[]
)
