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
    attr.StringIdentifier("country"),
    attr.StringIdentifier("country_code"),
    attr.StringIdentifier("continent"),
    attr.NaturalNumber("population"),
    attr.StringIdentifier("indicator"),
    attr.NaturalNumber("weekly_count"),
    attr.DateString("year_week"),
    attr.FloatLatitude("rate_14_day"),
    attr.NaturalNumber("cumulative_count"),
    attr.FreeText("source"),
])
# A row is equivalent to a struct
covid_euro_ts_datum = RowStruct(
    name="covid_euro_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("https://opendata.ecdc.europa.eu/covid19/nationalcasedeath/csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="covid_euro",
    schema=default_tabular_schema(covid_euro_ts_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="covid_euro",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
ECDC_covid_dataset = DataSet(
    name="covid_euro",
    description=(
        "Contains information on the 14-day notification rate of"
        "newly reported COVID-19 cases per 100 000 population"
        "and the 14-day notification rate of reported deaths per"
        "million populatoin by week and country."
    ),
    sourcePath=__file__,
    datumTemplates=[
        covid_euro_ts_datum,
    ],
    assets={
        "ECDC_COVID-19_data": table,
    },
)
