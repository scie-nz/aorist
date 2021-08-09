from aorist import (
    RowStruct,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorageSetup,
    StaticDataTable,
    DataSet,
    default_tabular_schema,
    attr_list,
    WebLocation,
    RemoteStorage,
    CSVHeader,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining dataset
"""
# Attributes in the dataset
attributes = attr_list([
     attr.StringIdentifier("GameId"),
     attr.DateString("GameDate"),
     attr.NaturalNumber("Quarter"),
     attr.NaturalNumber("Minute"),
     attr.NaturalNumber("Second"),
     attr.StringIdentifier("OffenseTeam"),
     attr.StringIdentifier("DefenseTeam"),
     attr.NaturalNumber("Down"),
     attr.NaturalNumber("ToGo"),
     attr.NaturalNumber("YardLine"),
     attr.Empty(""),
     attr.NaturalNumber("SeriesFirstDown"),
     attr.Empty(""),
     attr.NaturalNumber("NextScore"),
     attr.FreeText("Description"),
     attr.NaturalNumber("TeamWin"),
     attr.Empty(""),
     attr.Empty(""),
     attr.Year("SeasonYear"),
     attr.IntegerNumber("Yards"),
     attr.StringIdentifier("Formation"),
     attr.StringIdentifier("PlayType"),
     attr.NaturalNumber("IsRush"),
     attr.NaturalNumber("IsPass"),
     attr.NaturalNumber("IsIncomplete"),
     attr.NaturalNumber("IsTouchdown"),
     attr.StringIdentifier("PassType"),
     attr.NaturalNumber("IsSack"),
     attr.NaturalNumber("IsChallenge"),
     attr.NaturalNumber("IsChallengeReversed"),
     attr.Empty("Challenger"),
     attr.NaturalNumber("IsMeasurement"),
     attr.NaturalNumber("IsInterception"),
     attr.NaturalNumber("IsFumble"),
     attr.NaturalNumber("IsPenalty"),
     attr.NaturalNumber("IsTwoPointConversion"),
     attr.NaturalNumber("IsTwoPointConversionSuccessful"),
     attr.StringIdentifier("RushDirection"),
     attr.NaturalNumber("YardLineFixed"),
     attr.StringIdentifier("YardLineDirection"),
     attr.NaturalNumber("IsPenaltyAccepted"),
     attr.StringIdentifier("PenaltyTeam"),
     attr.NaturalNumber("IsNoPlay"),
     attr.StringIdentifier("PenaltyType"),
     attr.NaturalNumber("PenaltyYards"),
])

# A row is equivalent to a struct
nfl_2020_pbp_datum = RowStruct(
    name="nfl_2020_pbp_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("http://nflsavant.com/pbp_data.php?year=2020"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="nfl_2020_pbp_table",
    schema=default_tabular_schema(nfl_2020_pbp_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="nfl_2020_play_by_play",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
nfl_2020_pbp_dataset = DataSet(
    name="nfl_2020_pbp_dataset",
    description=(
        "2020 NFL play-by-play data."
    ),
    sourcePath=__file__,
    datumTemplates=[
        nfl_2020_pbp_datum,
    ],
    assets={
        "NFL_2020_play_by_play_data": table,
    },
)
