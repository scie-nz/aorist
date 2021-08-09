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
     attr.NaturalNumber(""),
     attr.NaturalNumber("EVENTMSGACTIONTYPE"),
     attr.NaturalNumber("EVENTMSGTYPE"),
     attr.NaturalNumber("EVENTNUM"),
     attr.StringIdentifier("GAME_ID"),
     attr.FreeText("HOMEDESCRIPTION"),
     attr.Empty("NEUTRALDESCRIPTION"),
     attr.ISO8601Timestamp("PCTIMESTRING"),
     attr.NaturalNumber("PERIOD"),
     attr.NaturalNumber("PERSON1TYPE"),
     attr.NaturalNumber("PERSON2TYPE"),
     attr.NaturalNumber("PERSON3TYPE"),
     attr.StringIdentifier("PLAYER1_ID"),
     attr.StringIdentifier("PLAYER1_NAME"),
     attr.StringIdentifier("PLAYER1_TEAM_ABBREVIATION"),
     attr.StringIdentifier("PLAYER1_TEAM_CITY"),
     attr.StringIdentifier("PLAYER1_TEAM_ID"),
     attr.StringIdentifier("PLAYER1_TEAM_NICKNAME"),
     attr.StringIdentifier("PLAYER2_ID"),
     attr.StringIdentifier("PLAYER2_NAME"),
     attr.StringIdentifier("PLAYER2_TEAM_ABBREVIATION"),
     attr.StringIdentifier("PLAYER2_TEAM_CITY"),
     attr.StringIdentifier("PLAYER2_TEAM_ID"),
     attr.StringIdentifier("PLAYER2_TEAM_NICKNAME"),
     attr.StringIdentifier("PLAYER3_ID"),
     attr.StringIdentifier("PLAYER3_NAME"),
     attr.StringIdentifier("PLAYER3_TEAM_ABBREVIATION"),
     attr.StringIdentifier("PLAYER3_TEAM_CITY"),
     attr.StringIdentifier("PLAYER3_TEAM_ID"),
     attr.StringIdentifier("PLAYER3_TEAM_NICKNAME"),
     attr.StringIdentifier("SCORE"),
     attr.NaturalNumber("SCOREMARGIN"),
     attr.FreeText("VISITORDESCRIPTION"),
     attr.ISO8601Timestamp("WCTIMESTRING"),
])

# A row is equivalent to a struct
nba_2020_pbp_datum = RowStruct(
    name="nba_2020_pbp_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("https://eightthirtyfour.com/nba/pbp/2018-19_pbp.csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="nba_2020_pbp_table",
    schema=default_tabular_schema(nba_2020_pbp_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="nba_2020_play_by_play",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
nba_2020_pbp_dataset = DataSet(
    name="nba_2020_pbp_dataset",
    description=(
        "Play by play data for the 2020 NBA season. This set of data has"
        " some additional columns, such as the current players on the court,"
        " which is derived from a combination of the box score and in game"
        " substitution data. Additonal data, parsed and derived from the"
        " play by play, such as possession tracking and event parsing is"
        " contained in this set."
    ),
    sourcePath=__file__,
    datumTemplates=[
        nba_2020_pbp_datum,
    ],
    assets={
        "NBA_play_by_play_data": table,
    },
)
