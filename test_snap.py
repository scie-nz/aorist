import inspect
import copy
from aorist import *
from recipes import programs
from common import endpoints

tmp_dir = "tmp/snap"

attributes = [
    Attribute(NumericIdentifier("from_id")),
    Attribute(NumericIdentifier("to_id")),
]
edge_tuple = IdentifierTuple(
    name="edge",
    attributes=attributes,
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
tables = {}
for name in names:

    name_underscore = name.replace("-", "_").lower()
    remote = RemoteStorage(
        location=RemoteLocation(WebLocation(
            address="https://snap.stanford.edu/data/%s.txt.gz" % name,
        )),
        layout=APIOrFileLayout(FileBasedStorageLayout(SingleFileLayout())),
        encoding=Encoding(TSVEncoding(
            compression=DataCompression(GzipCompression()),
            header=FileHeader(CSVHeader(num_lines=4)),
        )),
    )
    local = HiveTableStorage(
        location=HiveLocation(MinioLocation(name=name_underscore)),
        layout=TabularLayout(StaticTabularLayout()),
        encoding=Encoding(ORCEncoding()),
    )
    table = Asset(StaticDataTable(
        name=name_underscore,
        schema=default_tabular_schema(edge_tuple, name, attributes),
        setup=StorageSetup(RemoteStorageSetup(
            remote=Storage(remote),
        )),
        tag=name_underscore,
    ))
    tables[name] = table

snap_dataset = DataSet(
    name="snap",
    description=(
        "Sample datasets from the [Stanford Network Analysis Platform]"
        "(http://snap.stanford.edu/data/index.html)."
    ),
    source_path=__file__,
    datum_templates=[DatumTemplate(edge_tuple)],
    assets=tables,
    tag="snap",
    access_policies=[],
)


"""
Dataset will be replicated.
"""
snap_dataset = snap_dataset.replicate_to_local(
    Storage(local),
    tmp_dir,
    Encoding(CSVEncoding())
)
universe = Universe(
    name="my_cluster",
    datasets=[snap_dataset],
    endpoints=endpoints,
)
universe.compute_uuids()

result = dag(
    universe,
    ["DownloadDataFromRemoteWebLocation"],
    "python",
    programs,
    dialect_preferences=[
        R(),
        Python([]),
        Bash(),
        Presto(),
    ],
)
print(result)
