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
    attr.DateString("Date"),
    attr.StringIdentifier("State"),
    attr.NaturalNumber("Cases_Total"),
    attr.NaturalNumber("Cases_White"),
    attr.NaturalNumber("Cases_Black"),
    attr.NaturalNumber("Cases_Latinx"),
    attr.NaturalNumber("Cases_Asian"),
    attr.NaturalNumber("Cases_AIAN"),
    attr.NaturalNumber("Cases_NHPI"),
    attr.NaturalNumber("Cases_Multiracial"),
    attr.NaturalNumber("Cases_Other"),
    attr.NaturalNumber("Cases_Unknown"),
    attr.NaturalNumber("Cases_Ethnicity_Hispanic"),
    attr.NaturalNumber("Cases_Ethnicity_NonHispanic"),
    attr.NaturalNumber("Cases_Ethnicity_Unknown"),
    attr.NaturalNumber("Deaths_Total"),
    attr.NaturalNumber("Deaths_White"),
    attr.NaturalNumber("Deaths_Black"),
    attr.NaturalNumber("Deaths_Latinx"),
    attr.NaturalNumber("Deaths_Asian"),
    attr.NaturalNumber("Deaths_AIAN"),
    attr.NaturalNumber("Deaths_NHPI"),
    attr.NaturalNumber("Deaths_Multiracial"),
    attr.NaturalNumber("Deaths_Other"),
    attr.NaturalNumber("Deaths_Unknown"),
    attr.NaturalNumber("Deaths_Ethnicity_Hispanic"),
    attr.NaturalNumber("Deaths_Ethnicity_NonHispanic"),
    attr.NaturalNumber("Deaths_Ethnicity_Unknown"),
    attr.NaturalNumber("Hosp_Total"),
    attr.NaturalNumber("Hosp_White"),
    attr.NaturalNumber("Hosp_Black"),
    attr.NaturalNumber("Hosp_Latinx"),
    attr.NaturalNumber("Hosp_Asian"),
    attr.NaturalNumber("Hosp_AIAN"),
    attr.NaturalNumber("Hosp_NHPI"),
    attr.NaturalNumber("Hosp_Multiracial"),
    attr.NaturalNumber("Hosp_Other"),
    attr.NaturalNumber("Hosp_Unknown"),
    attr.NaturalNumber("Hosp_Ethnicity_Hispanic"),
    attr.NaturalNumber("Hosp_Ethnicity_NonHispanic"),
    attr.NaturalNumber("Hosp_Ethnicity_Unknown"),
    attr.NaturalNumber("Tests_Total"),
    attr.NaturalNumber("Tests_White"),
    attr.NaturalNumber("Tests_Black"),
    attr.NaturalNumber("Tests_Latinx"),
    attr.NaturalNumber("Tests_Asian"),
    attr.NaturalNumber("Tests_AIAN"),
    attr.NaturalNumber("Tests_NHPI"),
    attr.NaturalNumber("Tests_Multiracial"),
    attr.NaturalNumber("Tests_Other"),
    attr.NaturalNumber("Tests_Unknown"),
    attr.NaturalNumber("Tests_Ethnicity_Hispanic"),
    attr.NaturalNumber("Tests_Ethnicity_NonHispanic"),
    attr.NaturalNumber("Tests_Ethnicity_Unknown"),
])
# A row is equivalent to a struct
trcdt_datum = RowStruct(
    name="the_covid_racial_data_tracker_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=WebLocation(
        address=(
            "https://docs.google.com/spreadsheets/d/e/2PACX-1vS8SzaERcKJOD"
            "_EzrtCDK1dX1zkoMochlA9iHoHg_RSw3V8bkpfk1mpw4pfL5RdtSOyx_oScsUt"
            "yXyk/pub?gid=43720681&single=true&output=csv"
        ),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
table = StaticDataTable(
    name="covid_racial_data_tracker",
    schema=default_tabular_schema(trcdt_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="racial-covid-tracker",
)


# Our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
trcdt_dataset = DataSet(
    name="The-covid-racial-data-tracker",
    description=(
        "The COVID Racial Data Tracker is a collaboration between the COVID"
        " Tracking Project and the Boston University Center for Antiracist" 
        " Research. Together, they’re gathering the most complete and up-to"
        "-date race and ethnicity data on COVID-19 in the United States."
    ),
    sourcePath=__file__,
    datumTemplates=[
        trcdt_datum,
    ],
    assets={
        "The COVID Racial Data Tracker data": table,
    },
)
