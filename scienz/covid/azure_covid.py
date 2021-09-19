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
    attr.DateString("date"),
    attr.StringIdentifier("state"),
    attr.NaturalNumber("positive"),
    attr.NaturalNumber("negative"),
    attr.NaturalNumber("pending"),
    attr.NaturalNumber("hospitalized_currently"),
    attr.NaturalNumber("hospitalized_cumulative"),
    attr.NaturalNumber("in_icu_currently"),
    attr.NaturalNumber("in_icu_cumulative"),
    attr.NaturalNumber("on_ventilator_currently"),
    attr.NaturalNumber("on_ventilator_cumultive"),
    attr.NaturalNumber("recovered"),
    attr.Empty("data_quality_grade"),
    attr.DateString("last_update_et"),
    attr.FreeText("hash"),
    attr.DateString("date_checked"),
    attr.NaturalNumber("death"),
    attr.NaturalNumber("hospitalized"),
    attr.NaturalNumber("total"),
    attr.NaturalNumber("total_test_results"),
    attr.NaturalNumber("pos_neg"),
    attr.FIPSStateCode("fips"),
    attr.IntegerNumber("death_increase"),
    attr.IntegerNumber("hospitalized_increase"),
    attr.IntegerNumber("negative_increase"),
    attr.IntegerNumber("positive_increase"),
    attr.IntegerNumber("total_test_results_increase"),
    attr.FIPSStateCode("fips_code"),
    attr.StringIdentifier("iso_subdivision"),
    attr.DateString("load_time"),
    attr.StringIdentifier("iso_country"),
])
# A row is equivalent to a struct
microsoft_covid_ts_datum = RowStruct(
    name="microsoft_covid_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=(
            "https://pandemicdatalake.blob.core.windows.net/public/"
            "curated/covid-19/covid_tracking/latest/covid_tracking.csv"
        ),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="azure-covid",
    schema=default_tabular_schema(microsoft_covid_ts_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="azure-covid",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
azure_covid_dataset = DataSet(
    name="azure-covid",
    description=(
        "Statistics about COVID 19 in the United States compiled by Microsoft,"
        "including test, confirmed cases, hospitalizations, and patient "
        "outcomes from every US state and territory, up until 2021-03-07."
    ),
    sourcePath=__file__,
    datumTemplates=[
        microsoft_covid_ts_datum,
    ],
    assets={
        "Microsoft COVID-19 data": table,
    },
)
