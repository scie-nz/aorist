from aorist import (
    Attribute,
    NaturalNumber,
    StringIdentifier,
    DateString,
    POSIXTimestamp,
    PositiveFloat,
    default_tabular_schema,
    RowStruct,
    StaticDataTable,
    DataSchema,
    StorageSetup,
    RemoteStorageSetup,
    Storage,
    RemoteStorage,
    RemoteLocation,
    CSVEncoding,
    Encoding,
    DataSet,
    DatumTemplate,
    Asset,
    GithubLocation,
    FileBasedStorageLayout,
    CSVHeader,
    FileHeader,
    APIOrFileLayout,
    SingleFileLayout,
    FreeText,
    Empty,
    FIPSStateCode,
    IntegerNumber,
    RegionName,
    FIPSCountyCode,
    IPEDSID,
    USHigherEdName,
    Proportion,
    CountryName,
    Categorical,
    Year,
    Month,
    Week,
)

covid_ts_attributes = [
    Attribute(DateString("date")),
    Attribute(NaturalNumber("cases")),
    Attribute(NaturalNumber("deaths")),
]

covid_ts_datum = RowStruct(
    name="covid_ts_datum",
    attributes=covid_ts_attributes,
)

covid_ts_schema = default_tabular_schema(
    DatumTemplate(covid_ts_datum), covid_ts_attributes
)

us = Asset(StaticDataTable(
    name="us",
    schema=DataSchema(covid_ts_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),            
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),    
        ))
    )),
    tag="covid",
))

covid_state_attributes=[
    Attribute(DateString("date")),
    Attribute(RegionName("state")),
    Attribute(FIPSStateCode("fips")),
    Attribute(NaturalNumber("cases")),
    Attribute(NaturalNumber("deaths")),
]

covid_state_datum = RowStruct(
    name="covid_state_datum",
    attributes=covid_state_attributes,
)

covid_state_schema = default_tabular_schema(
    DatumTemplate(covid_state_datum), covid_state_attributes
)

states = Asset(StaticDataTable(
    name="us_states",
    schema=DataSchema(covid_state_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us-state.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),
        ))
    )),
    tag="covid_state",
))

covid_county_attributes=[
    Attribute(DateString("date")),
    Attribute(RegionName("state")),
    Attribute(RegionName("county")),
    Attribute(FIPSCountyCode("fips")),
    Attribute(NaturalNumber("cases")),
    Attribute(NaturalNumber("deaths")),
]

covid_county_datum = RowStruct(
    name="covid_county_datum",
    attributes=covid_county_attributes,
)

covid_county_schema = default_tabular_schema(
    DatumTemplate(covid_county_datum), covid_county_attributes
)

counties = Asset(StaticDataTable(
    name="us_counties",
    schema=DataSchema(covid_county_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us-counties.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),
        ))
    )),
    tag="covid_counties",
))

covid_college_attributes=[
    Attribute(DateString("date")),
    Attribute(RegionName("state")),
    Attribute(RegionName("county")),
    Attribute(RegionName("city")),
    Attribute(IPEDSID("ipeds_id")),
    Attribute(USHigherEdName("college")),
    Attribute(NaturalNumber("cases")),
    Attribute(NaturalNumber("cases_2021")),
    Attribute(FreeText("notes")),
]

covid_college_datum = RowStruct(
    name="covid_college_datum",
    attributes=covid_college_attributes,
)

covid_college_schema = default_tabular_schema(
    DatumTemplate(covid_college_datum), covid_college_attributes
)

colleges = Asset(StaticDataTable(
    name="us_colleges",
    schema=DataSchema(covid_college_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="us-colleges.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),
        ))
    )),
    tag="covid_colleges",
))

covid_mask_use_attributes=[
    Attribute(FIPSCountyCode("countyfp")),
    Attribute(Proportion("never")),
    Attribute(Proportion("rarely")),
    Attribute(Proportion("sometimes")),
    Attribute(Proportion("frequently")),
    Attribute(Proportion("always")),
]

covid_mask_use_datum = RowStruct(
    name="covid_mask_use_datum",
    attributes=covid_mask_use_attributes,
)

covid_mask_use_schema = default_tabular_schema(
    DatumTemplate(covid_mask_use_datum), covid_mask_use_attributes
)

mask_use = Asset(StaticDataTable(
    name="us_mask_use",
    schema=DataSchema(covid_mask_use_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="mask-use/mask-use-by-county.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),
        ))
    )),
    tag="covid_mask_use",
))


covid_excess_deaths_attributes=[
    Attribute(CountryName("country")),
    Attribute(RegionName("placename")),
    Attribute(Categorical("frequency")),
    Attribute(DateString("start_date")),
    Attribute(DateString("ned_date")),
    Attribute(Year("year")),
    Attribute(Month("month")),
    Attribute(Week("week")),
    Attribute(NaturalNumber("deaths")),
    Attribute(NaturalNumber("expected_deaths")),
    Attribute(IntegerNumber("excess_deaths")),
    Attribute(FreeText("baseline")),
]

covid_excess_deaths_datum = RowStruct(
    name="covid_excess_deaths_datum",
    attributes=covid_excess_deaths_attributes,
)

covid_excess_deaths_schema = default_tabular_schema(
    DatumTemplate(covid_excess_deaths_datum), covid_excess_deaths_attributes
)

excess_deaths = Asset(StaticDataTable(
    name="excess_deaths",
    schema=DataSchema(covid_excess_deaths_schema),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(GithubLocation(
                organization="nytimes",
                repository="covid-19-data",
                path="excess-deaths/deaths.csv",
                branch="master",
            )),
            layout=APIOrFileLayout(
                FileBasedStorageLayout(
                    SingleFileLayout()
                ),
            ),
            encoding=Encoding(CSVEncoding(header=FileHeader(
                CSVHeader(num_lines=1)
            ))),
        ))
    )),
    tag="excess_deaths",
))

covid_dataset = DataSet(
    name="nyt-covid",
    description="""
        Statistics about COVID 19 in the United States compiled by the
         NYTimes.
    """,
    source_path=__file__,
    datum_templates=[
        DatumTemplate(covid_ts_datum),
        DatumTemplate(covid_state_datum),
        DatumTemplate(covid_county_datum),
        DatumTemplate(covid_college_datum),
        DatumTemplate(covid_mask_use_datum),
        DatumTemplate(covid_excess_deaths_datum),
    ],
    assets={
        "us": us,
        "us_states": states,
        "us_counties": counties,
        "us_colleges": colleges,
        "us_mask_use": mask_use,
        "excess_deaths": excess_deaths,
    },
    access_policies=[]
)
