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
    attr.StringIdentifier("Direction"),
    attr.Year("Year"),
    attr.DateString("Date"),
    attr.StringIdentifier("Weekday"),
    attr.CountryName("Country"),
    attr.StringIdentifier("Comodity"),
    attr.StringIdentifier("Transport_Mode"),
    attr.StringIdentifier("Measure"),
    attr.NaturalNumber("Value"),
    attr.NaturalNumber("Cumulative"),
])

# A row is equivalent to a struct
statsnz_covid_ts_datum = RowStruct(
    name="statsnz_covid_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=("https://www.stats.govt.nz/assets/Uploads/Effects-of-"
                 "COVID-19-on-trade/Effects-of-COVID-19-on-trade-At-"
                 "14-April-2021-provisional/Download-data/effects-of"
                 "-covid-19-on-trade-at-14-April-2021-provisional.csv"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="statsnz_covid",
    schema=default_tabular_schema(statsnz_covid_ts_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="statsnz_covid_effects_on_trade",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
statsnz_covid_dataset = DataSet(
    name="statsnz_covid",
    description=(
        "New Zealand's daily goods trade with the world. Comparing" 
        "values with previous years shows the potential impacts of COVID-19."
    ),
    sourcePath=__file__,
    datumTemplates=[
        statsnz_covid_ts_datum,
    ],
    assets={
        "StatsNZ_effects_of_COVID-19_on_trade_data": table,
    },
)
