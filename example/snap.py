from aorist import (
    AlluxioLocation,
    WebLocation,
    StaticHiveTableLayout,
    UpperSnakeCaseCSVHeader,
    GzipCompression,
    ORCEncoding,
    TSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteImportStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
    IdentifierTuple,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

edge_tuple = IdentifierTuple(
    name="edge",
    attributes=[
        attr.NumericIdentifier("from_id"),
        attr.NumericIdentifier("to_id"),
    ],
)
names = [
    "ca-AstroPh",
    "ca-CondMat",
    "ca-GrQc",
    "ca-HepPh",
    "ca-HepTh",
    "web-BerkStan",
    "web-Google",
    "web-NotreDame",
    "web-Stanford",
    "amazon0302",
    "amazon0312",
    "amazon0505",
    "amazon0601",
]
tables = []
for name in names:

    name_underscore = name.replace("-", "_").lower()
    remote = RemoteStorage(
        location=WebLocation(
            address="https://snap.stanford.edu/data/%s.txt.gz" % name,
        ),
        layout=SingleFileLayout(),
        encoding=TSVEncoding(
            compression=GzipCompression(),
            header=UpperSnakeCaseCSVHeader(num_lines=4),
        ),
    )
    local = HiveTableStorage(
        location=AlluxioLocation("snap_data", name_underscore),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    )
    table = StaticDataTable(
        name=name_underscore,
        schema=default_tabular_schema(edge_tuple),
        setup=RemoteImportStorageSetup(
            tmp_dir="/tmp/%s" % name_underscore,
            remote=remote,
            local=[local],
        ),
        tag=name_underscore,
    )
    tables += [table]

snap_dataset = DataSet(
    name="snap-dataset",
    datumTemplates=[edge_tuple],
    assets=tables,
    tag="snap",
)
