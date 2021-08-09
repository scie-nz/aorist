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
     attr.StringIdentifier("id"),
     attr.StringIdentifier("home_team"),
     attr.NaturalNumber("home_score"),
     attr.StringIdentifier("away_team"),
     attr.NaturalNumber("away_score"),
     attr.NaturalNumber("temperature"),
     attr.NaturalNumber("wind_chill"),
     attr.StringIdentifier("humidity"),
     attr.NaturalNumber("wind_mph"),
     attr.StringIdentifier("weather"),
     attr.DateString("date"),
])

# A row is equivalent to a struct
nfl_weather_datum = RowStruct(
    name="nfl_weather_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("http://nflsavant.com/dump/weather_20131231.csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="nfl_weather_table",
    schema=default_tabular_schema(nfl_weather_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="nfl_weather",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
nfl_weather_dataset = DataSet(
    name="nfl_weather_dataset",
    description=(
        "Data about the Weather for NFL games 1960-2013. From NFLsavant.com."
    ),
    sourcePath=__file__,
    datumTemplates=[
        nfl_weather_datum,
    ],
    assets={
        "nfl_weather_data": table,
    },
)
