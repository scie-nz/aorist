from aorist import (
    RowStruct,
    WebLocation,
    StaticTabularLayout,
    ZipCompression,
    ORCEncoding,
    CSVEncoding,
    GDBEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    ReplicationStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
    S3Location,
    attr_list,
    Universe,
)
from aorist import attributes as attr
from common import DEFAULT_ENDPOINTS

attributes = attr_list([
    attr.WKTString("wkt"),
    attr.FloatNumber("saf_cover_type"),
    attr.NaturalNumber("srm_cover_type"),
    attr.FreeText("regional_dominance_type_1"),
    attr.FreeText("os_tree_diameter_class_1"),
    attr.FreeText("tree_cfa_class_1"),
    attr.FreeText("regional_dominance_type_2"),
    attr.FreeText("os_tree_diameter_class_2"),
    attr.FreeText("regional_dominance_type_3"),
    attr.FreeText("covertype"),
    attr.FreeText("con_cfa"),
    attr.FreeText("hdw_cfa"),
    attr.NaturalNumber("shb_cfa"),
    attr.FreeText("total_tree_cfa"),
    attr.FreeText("whrlifeform"),
    attr.FreeText("whrtype"),
    attr.FloatNumber("whrsize"),
    attr.FreeText("whrdensity"),
    attr.IntegerNumber("ylf"),
    attr.FloatNumber("tslf"),
    attr.FreeText("last_fire_name"),
    attr.FloatNumber("num_fires"),
    attr.FloatNumber("num_fires_1970"),
    attr.FloatNumber("firesLast40"),
    attr.FreeText("PFR"),
    attr.FreeText("fireRegimeGrp"),
    attr.FloatNumber("currentFRI"),
    attr.FloatNumber("currentFRI_1970"),
    attr.FloatNumber("meanRefFRI"),
    attr.FloatNumber("medianRefFRI"),
    attr.FloatNumber("minRefFRI"),
    attr.FloatNumber("maxRefFRI"),
    attr.FloatNumber("meanPFRID"),
    attr.FloatNumber("meanPFRID_1970"),
    attr.FloatNumber("medianPFRID"),
    attr.FloatNumber("minPFRID"),
    attr.FloatNumber("maxPFRID"),
    attr.FloatNumber("meanCC_FRI"),
    attr.FloatNumber("meanCC_FRI_1970"),
    attr.FloatNumber("NPS_FRID"),
    attr.FreeText("NPS_FRID_Index"),
    attr.NaturalNumber("SHAPE_Length"),
    attr.FloatNumber("SHAPE_Area"),
])
frid_datum = RowStruct(
    name="frid_datum",
    attributes=attributes,
)
remote = RemoteStorage(
    location=WebLocation(
        address=("http://www.fs.fed.us/r5/rsl/projects/gis/data/"
                 "FRID/FRID_NorthCoast19_1_West.gdb.zip"),
    ),
    layout=SingleFileLayout(),
    encoding=GDBEncoding(
        compression=ZipCompression(filename="FRID_NorthCoast19_1_West.gdb"),
    ),
)
local = HiveTableStorage(
    location=S3Location(
        bucket="vibrant-dragon",
        key="datascience/frid",
    ),
    layout=StaticTabularLayout(),
    encoding=ORCEncoding(),
)
frid_table = StaticDataTable(
    name="frid_table",
    schema=default_tabular_schema(frid_datum),
    setup=ReplicationStorageSetup(
        tmp_dir="/tmp/frid2",
        tmp_encoding=CSVEncoding(),
        source=remote,
        targets=[local],
    ),
    tag="frid",
)
frid_dataset = DataSet(
    name="frid-2-dataset",
    description="Satellite data from Sentinel-2 from Google GCS",
    sourcePath=__file__,
    datumTemplates=[frid_datum],
    assets={
        "frid": frid_table
    },
)
universe = Universe(
    name="my_cluster",
    datasets=[frid_dataset],
    endpoints=DEFAULT_ENDPOINTS,
)
#print(universe.python("ConvertGDBToCSV"))
print(universe.airflow("UploadDataToS3"))
