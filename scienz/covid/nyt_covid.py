from aorist import (
    RowStruct,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorageSetup,
    StaticDataTable,
    DataSet,
    default_tabular_schema,
    attr_list,
    GithubLocation,
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
    attr.NaturalNumber("cases"),
    attr.NaturalNumber("deaths"),
])
# A row is equivalent to a struct
covid_ts_datum = RowStruct(
    name="covid_ts_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=GithubLocation(
        organization="nytimes",
        repository="covid-19-data",
        path="us.csv",
        branch="master",
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
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


covid_state_datum = RowStruct(
    name="covid_state_datum",
    attributes=attr_list([
        attr.DateString("date"),
        attr.RegionName("state"),
        attr.FIPSStateCode("fips"),
        attr.NaturalNumber("cases"),
        attr.NaturalNumber("deaths"),
    ])
)
states = StaticDataTable(
    name="us_states",
    schema=default_tabular_schema(covid_state_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us-states.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="covid_state",
)

covid_county_datum = RowStruct(
    name="covid_county_datum",
    attributes=attr_list([
        attr.DateString("date"),
        attr.RegionName("state"),
        attr.RegionName("county"),
        attr.FIPSCountyCode("fips"),
        attr.NaturalNumber("cases"),
        attr.NaturalNumber("deaths"),
    ])
)
counties = StaticDataTable(
    name="us_counties",
    schema=default_tabular_schema(covid_county_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us-counties.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="covid_county",
)

covid_college_datum = RowStruct(
    name="covid_college_datum",
    attributes=attr_list([
        attr.DateString("date"),
        attr.RegionName("state"),
        attr.RegionName("county"),
        attr.RegionName("city"),
        attr.IPEDSID("ipeds_id"),
        attr.USHigherEdName("college"),
        attr.NaturalNumber("cases"),
        attr.NaturalNumber("cases_2021"),
        attr.FreeText("notes"),
    ])
)
colleges = StaticDataTable(
    name="us_colleges",
    schema=default_tabular_schema(covid_college_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="colleges/colleges.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="covid_college",
)

covid_mask_use_datum = RowStruct(
    name="covid_mask_use_datum",
    attributes=attr_list([
        attr.FIPSCountyCode("countyfp"),
        attr.Proportion("never"),
        attr.Proportion("rarely"),
        attr.Proportion("sometimes"),
        attr.Proportion("frequently"),
        attr.Proportion("always"),
    ])
)
mask_use = StaticDataTable(
    name="us_mask_use",
    schema=default_tabular_schema(covid_mask_use_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="mask-use/mask-use-by-county.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="covid_mask_use",
)

covid_excess_deaths_datum = RowStruct(
    name="covid_excess_deaths_datum",
    attributes=attr_list([
        attr.CountryName("country"),
        attr.RegionName("placename"),
        attr.Categorical("frequency"),
        attr.DateString("start_date"),
        attr.DateString("ned_date"),
        attr.Year("year"),
        attr.Month("month"),
        attr.Week("week"),
        attr.NaturalNumber("deaths"),
        attr.NaturalNumber("expected_deaths"),
        attr.IntegerNumber("excess_deaths"),
        attr.FreeText("baseline"),
    ])
)
excess_deaths = StaticDataTable(
    name="excess_deaths",
    schema=default_tabular_schema(covid_excess_deaths_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="excess-deaths/deaths.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="covid_excess_deaths",
)

# our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
covid_dataset = DataSet(
    name="nyt-covid",
    description=(
        "Statistics about COVID 19 in the United States compiled by the"
        " NYTimes."
    ),
    sourcePath=__file__,
    datumTemplates=[
        covid_ts_datum,
        covid_state_datum,
        covid_county_datum,
        covid_college_datum,
        covid_mask_use_datum,
        covid_excess_deaths_datum,
    ],
    assets={
        "us": us,
        "us_states": states,
        "us_counties": counties,
        "us_colleges": colleges,
        "us_mask_use": mask_use,
        "excess_deaths": excess_deaths,
    },
)
