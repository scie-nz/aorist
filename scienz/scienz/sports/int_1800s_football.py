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
)

attributes1800 = (1800,1872,1899,[
    Attribute(FreeText("Date")),
    Attribute(StringIdentifier("Team 1")),
    Attribute(StringIdentifier("Score")),
    Attribute(StringIdentifier("Team 2")),
    Attribute(StringIdentifier("Tournament")),
    Attribute(FreeText("City")),
])

attr_list = [
    attributes1800,
]

templates = {}

for (century, start_year, end_year, attributes) in attr_list:
    key=(start_year, end_year)
    templates[key]=DatumTemplate(RowStruct(
        name="cache_internationals_%ds_datum" % (century),
        attributes=attributes,
    ))

def build_assets(century, start_year, end_year, attributes):
    return {"international_football_%d" % (x): Asset(StaticDataTable(
        name="SSEPL%d" % (x),
        schema=DataSchema(default_tabular_schema(
            templates[(start_year,end_year)],
            attributes=templates[(start_year,end_year)].attributes()
        )),
        setup=StorageSetup(RemoteStorageSetup(
            remote=Storage(RemoteStorage(
                location=RemoteLocation(GithubLocation(
                    organization="footballcsv",
                    repository="cache.internationals",
                    path="%ds/%d.csv" % (century, x),
                    branch="master",
                )),
                layout=APIOrFileLayout(FileBasedStorageLayout(
                    SingleFileLayout()
                )),
                encoding=Encoding(CSVEncoding(header=FileHeader(
                    CSVHeader(num_lines=1
                    )
                ))),
            )),
        )),
        tag=str(x)+"international_football",
        )) for x in range(start_year,end_year)}

assets1800s = build_assets(1800, 1872, 1899, attributes1800)

int_1800s_football_dataset = DataSet(
    name="int-1800-football",
    description="""
        International Football Results from 1872-1899 
         (https://github.com/footballcsv/cache.internationals)
    """,
    source_path=__file__,
    datum_templates=list(templates.values()),
    assets=assets1800s,
    access_policies=[]
)
