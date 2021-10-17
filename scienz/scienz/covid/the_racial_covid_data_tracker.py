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
    WebLocation,
    FileBasedStorageLayout,
    CSVHeader,
    FileHeader,
    APIOrFileLayout,
    SingleFileLayout,
    FreeText,
    Empty,
    FIPSStateCode,
    IntegerNumber,
)

attributes = [
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("State")),
    Attribute(NaturalNumber("Cases_Total")),
    Attribute(NaturalNumber("Cases_White")),
    Attribute(NaturalNumber("Cases_Black")),
    Attribute(NaturalNumber("Cases_Latinx")),
    Attribute(NaturalNumber("Cases_Asian")),
    Attribute(NaturalNumber("Cases_AIAN")),
    Attribute(NaturalNumber("Cases_NHPI")),
    Attribute(NaturalNumber("Cases_Multiracial")),
    Attribute(NaturalNumber("Cases_Other")),
    Attribute(NaturalNumber("Cases_Unknown")),
    Attribute(NaturalNumber("Cases_Ethnicity_Hispanic")),
    Attribute(NaturalNumber("Cases_Ethnicity_NonHispanic")),
    Attribute(NaturalNumber("Cases_Ethnicity_Unknown")),
    Attribute(NaturalNumber("Deaths_Total")),
    Attribute(NaturalNumber("Deaths_White")),
    Attribute(NaturalNumber("Deaths_Black")),
    Attribute(NaturalNumber("Deaths_Latinx")),
    Attribute(NaturalNumber("Deaths_Asian")),
    Attribute(NaturalNumber("Deaths_AIAN")),
    Attribute(NaturalNumber("Deaths_NHPI")),
    Attribute(NaturalNumber("Deaths_Multiracial")),
    Attribute(NaturalNumber("Deaths_Other")),
    Attribute(NaturalNumber("Deaths_Unknown")),
    Attribute(NaturalNumber("Deaths_Ethnicity_Hispanic")),
    Attribute(NaturalNumber("Deaths_Ethnicity_NonHispanic")),
    Attribute(NaturalNumber("Deaths_Ethnicity_Unknown")),
    Attribute(NaturalNumber("Hosp_Total")),
    Attribute(NaturalNumber("Hosp_White")),
    Attribute(NaturalNumber("Hosp_Black")),
    Attribute(NaturalNumber("Hosp_Latinx")),
    Attribute(NaturalNumber("Hosp_Asian")),
    Attribute(NaturalNumber("Hosp_AIAN")),
    Attribute(NaturalNumber("Hosp_NHPI")),
    Attribute(NaturalNumber("Hosp_Multiracial")),
    Attribute(NaturalNumber("Hosp_Other")),
    Attribute(NaturalNumber("Hosp_Unknown")),
    Attribute(NaturalNumber("Hosp_Ethnicity_Hispanic")),
    Attribute(NaturalNumber("Hosp_Ethnicity_NonHispanic")),
    Attribute(NaturalNumber("Hosp_Ethnicity_Unknown")),
    Attribute(NaturalNumber("Tests_Total")),
    Attribute(NaturalNumber("Tests_White")),
    Attribute(NaturalNumber("Tests_Black")),
    Attribute(NaturalNumber("Tests_Latinx")),
    Attribute(NaturalNumber("Tests_Asian")),
    Attribute(NaturalNumber("Tests_AIAN")),
    Attribute(NaturalNumber("Tests_NHPI")),
    Attribute(NaturalNumber("Tests_Multiracial")),
    Attribute(NaturalNumber("Tests_Other")),
    Attribute(NaturalNumber("Tests_Unknown")),
    Attribute(NaturalNumber("Tests_Ethnicity_Hispanic")),
    Attribute(NaturalNumber("Tests_Ethnicity_NonHispanic")),
    Attribute(NaturalNumber("Tests_Ethnicity_Unknown")),
]

trcdt_datum = RowStruct(
    name="the_racial_covid_data_tracker_datum",
    attributes=attributes,
)

trcdt_schema = default_tabular_schema(
    DatumTemplate(trcdt_datum), attributes
)

table = Asset(StaticDataTable(
            name="the_racial_covid_data_tracker_table",
            schema=DataSchema(trcdt_schema),
            setup=StorageSetup(RemoteStorageSetup(
                remote=Storage(RemoteStorage(
                    location=RemoteLocation(
                        WebLocation(
                            address=("https://docs.google.com/spreadsheets/d/e/2PACX-1vS8SzaERcKJOD"
                                     "_EzrtCDK1dX1zkoMochlA9iHoHg_RSw3V8bkpfk1mpw4pfL5RdtSOyx_oScsUt"
                                     "yXyk/pub?gid=43720681&single=true&output=csv"),
                        )
                    ),
                    layout=APIOrFileLayout(
                        FileBasedStorageLayout(
                            SingleFileLayout()
                        ),
                    ),
                    encoding=Encoding(CSVEncoding(header=FileHeader(
                        CSVHeader(num_lines=1)
                    ))),
                )),
            )),
            tag="the_racial_covid_data_tracker",
            ))

trcdt_dataset = DataSet(
    name="The-covid-racial-data-tracker",
    description="""
        The COVID Racial Data Tracker is a collaboration between the COVID
         Tracking Project and the Boston University Center for Antiracist 
         Research. Together, they’re gathering the most complete and up-to
        -date race and ethnicity data on COVID-19 in the United States.
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(trcdt_datum)],
    assets={
        "The COVID Racial Data Tracker data": table,
    },
    access_policies=[]
)
