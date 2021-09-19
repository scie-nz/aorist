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
    Empty,
    FreeText,
    IntegerNumber,
)

attributes = [
    Attribute(StringIdentifier("GameId")),
    Attribute(DateString("GameDate")),
    Attribute(NaturalNumber("Quarter")),
    Attribute(NaturalNumber("Minute")),
    Attribute(NaturalNumber("Second")),
    Attribute(StringIdentifier("OffenseTeam")),
    Attribute(StringIdentifier("DefenseTeam")),
    Attribute(NaturalNumber("Down")),
    Attribute(NaturalNumber("ToGo")),
    Attribute(NaturalNumber("YardLine")),
    Attribute(Empty("")),
    Attribute(NaturalNumber("SeriesFirstDown")),
    Attribute(Empty("")),
    Attribute(NaturalNumber("NextScore")),
    Attribute(FreeText("Description")),
    Attribute(NaturalNumber("TeamWin")),
    Attribute(Empty("")),
    Attribute(Empty("")),
    Attribute(Year("SeasonYear")),
    Attribute(IntegerNumber("Yards")),
    Attribute(StringIdentifier("Formation")),
    Attribute(StringIdentifier("PlayType")),
    Attribute(NaturalNumber("IsRush")),
    Attribute(NaturalNumber("IsPass")),
    Attribute(NaturalNumber("IsIncomplete")),
    Attribute(NaturalNumber("IsTouchdown")),
    Attribute(StringIdentifier("PassType")),
    Attribute(NaturalNumber("IsSack")),
    Attribute(NaturalNumber("IsChallenge")),
    Attribute(NaturalNumber("IsChallengeReversed")),
    Attribute(Empty("Challenger")),
    Attribute(NaturalNumber("IsMeasurement")),
    Attribute(NaturalNumber("IsInterception")),
    Attribute(NaturalNumber("IsFumble")),
    Attribute(NaturalNumber("IsPenalty")),
    Attribute(NaturalNumber("IsTwoPointConversion")),
    Attribute(NaturalNumber("IsTwoPointConversionSuccessful")),
    Attribute(StringIdentifier("RushDirection")),
    Attribute(NaturalNumber("YardLineFixed")),
    Attribute(StringIdentifier("YardLineDirection")),
    Attribute(NaturalNumber("IsPenaltyAccepted")),
    Attribute(StringIdentifier("PenaltyTeam")),
    Attribute(NaturalNumber("IsNoPlay")),
    Attribute(StringIdentifier("PenaltyType")),
    Attribute(NaturalNumber("PenaltyYards")),
]

nfl_2020_pbp_datum = RowStruct(
    name="nfl_2020_pbp_datum",
    attributes=attributes,
)

nfl_2020_pbp_schema = default_tabular_schema(
    DatumTemplate(nfl_2020_pbp_datum), attributes
)

table = Asset(StaticDataTable(
            name="nfl_2020_pbp_table",
            schema=DataSchema(nfl_2020_pbp_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("http://nflsavant.com/pbp_data.php?year=2020"),
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
            tag="nfl_play_by_play",
            ))

nfl_2020_pbp_dataset = DataSet(
    name="nfl_2020_pbp_dataset",
    description="""
        2020 NFL play-by-play data.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(nfl_2020_pbp_datum)],
    assets={
        "NFL_2020_play_by_play_data": table,
    },
    access_policies=[]
)
