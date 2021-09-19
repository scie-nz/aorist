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
    attr.DateString("Date_reported"),
    attr.StringIdentifier("Country_code"),
    attr.CountryName("Country"),
    attr.StringIdentifier("WHO_region"),
    attr.NaturalNumber("New_cases"),
    attr.NaturalNumber("Cumulative_cases"),
    attr.NaturalNumber("New_deaths"),
    attr.NaturalNumber("Cumulative_deaths"),
])

# A row is equivalent to a struct
who_covid_ts_datum = RowStruct(
    name="who_covid_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("https://covid19.who.int/WHO-COVID-19-global-data.csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="who_covid",
    schema=default_tabular_schema(who_covid_ts_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="who_covid",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
who_covid_dataset = DataSet(
    name="who_covid",
    description=(
        "COVID-19 new and cumulative cases and deaths by country, "
        "from the World Health Organisation"
    ),
    sourcePath=__file__,
    datumTemplates=[
        who_covid_ts_datum,
    ],
    assets={
        "World_Health_Organisation_COVID-19_data": table,
    },
)
