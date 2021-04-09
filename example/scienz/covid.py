from aorist import (
    RowStruct,
    MinioLocation,
    WebLocation,
    StaticTabularLayout,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteStorageSetup,
    StaticDataTable,
    DataSet,
    default_tabular_schema,
    attr_list,
    GithubLocation,
    GitStorage,
    CSVHeader,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining dataset
"""
# Attributes in the dataset
attributes = attr_list([
    attr.DateString("date"),
    attr.NaturalNumber("cases"),
    attr.NaturalNumber("cdeaths"),

])
# A row is equivalent to a struct
covid_ts_datum = RowStruct(
    name="covid_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = GitStorage(
    location=GithubLocation(
        organization="nytimes",
        repository="covid-19-data",
        path="us.csv",
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# This data is to be replicated locally
local = HiveTableStorage(
    location=MinioLocation(name="nyt-covid"),
    layout=StaticTabularLayout(),
    encoding=ORCEncoding(),
)
# We will create a table that will always have the same content
# (we do not expect it to change over time)
us = StaticDataTable(
    name="us",
    schema=default_tabular_schema(covid_ts_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="covid",
)
# our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
covid_dataset = DataSet(
    name="nyt-covid",
    description="Statistics about COVID 19 compiled by the NYTimes",
    sourcePath=__file__,
    datumTemplates=[covid_ts_datum],
    assets={"us": us},
)
