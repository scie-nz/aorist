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
     attr.StringIdentifier("Country"),
     attr.StringIdentifier("League"),
     attr.Year("Season"),
     attr.DateString("Date"),
     attr.POSIXTimestamp("Time"),
     attr.StringIdentifier("Home"),
     attr.StringIdentifier("Away"),
     attr.NaturalNumber("HG"),
     attr.NaturalNumber("AG"),
     attr.StringIdentifier("Res"),
     attr.PositiveFloat("PH"),
     attr.PositiveFloat("PD"),
     attr.PositiveFloat("PA"),
     attr.PositiveFloat("MaxH"),
     attr.PositiveFloat("MaxD"),
     attr.PositiveFloat("MaxA"),
     attr.PositiveFloat("AvgH"),
     attr.PositiveFloat("AvgD"),
     attr.PositiveFloat("AvgA"),
])

# A row is equivalent to a struct
mls_usa_datum = RowStruct(
    name="mls_usa_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("https://www.football-data.co.uk/new/USA.csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="mls_usa_table",
    schema=default_tabular_schema(mls_usa_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="mls_usa_football",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
mls_usa_dataset = DataSet(
    name="mls_usa_dataset",
    description=(
        "This dataset contains match statistics for Major League"
        " Soccer in the USA from 2012 onwards."
    ),
    sourcePath=__file__,
    datumTemplates=[
        mls_usa_datum,
    ],
    assets={
        "Major__League_Soccer_data": table,
    },
)
