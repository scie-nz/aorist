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
    ISO8601Timestamp,
)

attributes = [
    Attribute(NaturalNumber("")),
    Attribute(NaturalNumber("EVENTMSGACTIONTYPE")),
    Attribute(NaturalNumber("EVENTMSGTYPE")),
    Attribute(NaturalNumber("EVENTNUM")),
    Attribute(StringIdentifier("GAME_ID")),
    Attribute(FreeText("HOMEDESCRIPTION")),
    Attribute(Empty("NEUTRALDESCRIPTION")),
    Attribute(ISO8601Timestamp("PCTIMESTRING")),
    Attribute(NaturalNumber("PERIOD")),
    Attribute(NaturalNumber("PERSON1TYPE")),
    Attribute(NaturalNumber("PERSON2TYPE")),
    Attribute(NaturalNumber("PERSON3TYPE")),
    Attribute(StringIdentifier("PLAYER1_ID")),
    Attribute(StringIdentifier("PLAYER1_NAME")),
    Attribute(StringIdentifier("PLAYER1_TEAM_ABBREVIATION")),
    Attribute(StringIdentifier("PLAYER1_TEAM_CITY")),
    Attribute(StringIdentifier("PLAYER1_TEAM_ID")),
    Attribute(StringIdentifier("PLAYER1_TEAM_NICKNAME")),
    Attribute(StringIdentifier("PLAYER2_ID")),
    Attribute(StringIdentifier("PLAYER2_NAME")),
    Attribute(StringIdentifier("PLAYER2_TEAM_ABBREVIATION")),
    Attribute(StringIdentifier("PLAYER2_TEAM_CITY")),
    Attribute(StringIdentifier("PLAYER2_TEAM_ID")),
    Attribute(StringIdentifier("PLAYER2_TEAM_NICKNAME")),
    Attribute(StringIdentifier("PLAYER3_ID")),
    Attribute(StringIdentifier("PLAYER3_NAME")),
    Attribute(StringIdentifier("PLAYER3_TEAM_ABBREVIATION")),
    Attribute(StringIdentifier("PLAYER3_TEAM_CITY")),
    Attribute(StringIdentifier("PLAYER3_TEAM_ID")),
    Attribute(StringIdentifier("PLAYER3_TEAM_NICKNAME")),
    Attribute(StringIdentifier("SCORE")),
    Attribute(NaturalNumber("SCOREMARGIN")),
    Attribute(FreeText("VISITORDESCRIPTION")),
    Attribute(ISO8601Timestamp("WCTIMESTRING")),
]

nba_2020_pbp_datum = RowStruct(
    name="nba_2020_pbp_datum",
    attributes=attributes,
)

nba_2020_pbp_schema = default_tabular_schema(
    DatumTemplate(nba_2020_pbp_datum), attributes
)

table = Asset(StaticDataTable(
            name="nba_2020_pbp_table",
            schema=DataSchema(nba_2020_pbp_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://eightthirtyfour.com/nba/pbp/2018-19_pbp.csv"),
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
            tag="nba_play_by_play",
            ))



nba_2020_pbp_dataset = DataSet(
    name="nba_2020_pbp_dataset",
    description="""
        Play by play data for the 2020 NBA season. This set of data has
         some additional columns, such as the current players on the court,
         which is derived from a combination of the box score and in game
         substitution data. Additonal data, parsed and derived from the
         play by play, such as possession tracking and event parsing is
         contained in this set.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(nba_2020_pbp_datum)],
    assets={
        "NBA_play_by_play_data": table,
    },
    access_policies=[]
)
