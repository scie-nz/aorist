download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)


hive_schemas_created__amazon0302 = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS amazon0302 (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__amazon0302)
hive_schemas_created__amazon0505 = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS amazon0505 (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__amazon0505)
hive_schemas_created__web_Google = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS web_Google (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__web_Google)
hive_schemas_created__sentinel = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS sentinel-2-metadata-table (
    datatake_identifier VARCHAR,
    geometric_quality_flag VARCHAR,
    granule_id VARCHAR,
    generation_time BIGINT,
    south_lat REAL COMMENT \'Southern latitude of the tile`s bounding box.\',
    west_lon REAL COMMENT \'Western longitude of the tile`s bounding box.\',
    cloud_cover VARCHAR,
    base_url VARCHAR,
    product_id VARCHAR,
    sensing_time BIGINT,
    north_lat REAL COMMENT \'Northern latitude of the tile`s bounding box.\',
    east_lon REAL COMMENT \'Eastern longitude of the tile`s bounding box.\',
    total_size BIGINT,
    mgrs_tile VARCHAR
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__sentinel)
hive_schemas_created__ca_GrQc = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca_GrQc (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__ca_GrQc)
hive_schemas_created__ca_AstroPh = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca_AstroPh (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__ca_AstroPh)
hive_schemas_created__web_Stanford = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS web_Stanford (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__web_Stanford)
hive_schemas_created__ca_HepPh = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca_HepPh (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__ca_HepPh)
hive_schemas_created__ca_HepTh = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca_HepTh (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__ca_HepTh)
hive_schemas_created__web_BerkStan = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS web_BerkStan (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__web_BerkStan)
hive_schemas_created__amazon0601 = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS amazon0601 (
    from_id BIGINT,
    to_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__amazon0601)
hive_schemas_created__ca_CondMat = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS ca_CondMat (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__ca_CondMat)
hive_schemas_created__web_NotreDame = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS web_NotreDame (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__web_NotreDame)
hive_schemas_created__amazon0312 = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS amazon0312 (
    to_id BIGINT,
    from_id BIGINT
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_schemas_created__amazon0312)


replicated_schema__sentinel = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__sentinel)
flow.add_edge(hive_schemas_created__sentinel, replicated_schema__sentinel)
replicated_schema__ca_CondMat = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__ca_CondMat)
flow.add_edge(hive_schemas_created__ca_CondMat, replicated_schema__ca_CondMat)
replicated_schema__amazon0505 = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__amazon0505)
flow.add_edge(hive_schemas_created__amazon0505, replicated_schema__amazon0505)
replicated_schema__ca_GrQc = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__ca_GrQc)
flow.add_edge(hive_schemas_created__ca_GrQc, replicated_schema__ca_GrQc)
replicated_schema__web_Google = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__web_Google)
flow.add_edge(hive_schemas_created__web_Google, replicated_schema__web_Google)
replicated_schema__ca_HepTh = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__ca_HepTh)
flow.add_edge(hive_schemas_created__ca_HepTh, replicated_schema__ca_HepTh)
replicated_schema__amazon0302 = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__amazon0302)
flow.add_edge(hive_schemas_created__amazon0302, replicated_schema__amazon0302)
replicated_schema__ca_AstroPh = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__ca_AstroPh)
flow.add_edge(hive_schemas_created__ca_AstroPh, replicated_schema__ca_AstroPh)
replicated_schema__ca_HepPh = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__ca_HepPh)
flow.add_edge(hive_schemas_created__ca_HepPh, replicated_schema__ca_HepPh)
replicated_schema__web_Stanford = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__web_Stanford)
flow.add_edge(hive_schemas_created__web_Stanford,
              replicated_schema__web_Stanford)
replicated_schema__web_BerkStan = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__web_BerkStan)
flow.add_edge(hive_schemas_created__web_BerkStan,
              replicated_schema__web_BerkStan)
replicated_schema__web_NotreDame = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__web_NotreDame)
flow.add_edge(hive_schemas_created__web_NotreDame,
              replicated_schema__web_NotreDame)
replicated_schema__amazon0601 = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__amazon0601)
flow.add_edge(hive_schemas_created__amazon0601, replicated_schema__amazon0601)
replicated_schema__amazon0312 = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema__amazon0312)
flow.add_edge(hive_schemas_created__amazon0312, replicated_schema__amazon0312)


download_data_from_remote__amazon0601 = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__amazon0601)
download_data_from_remote__amazon0505 = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__amazon0505)
download_data_from_remote__ca_AstroPh = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__ca_AstroPh)
download_data_from_remote__ca_CondMat = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__ca_CondMat)
download_data_from_remote__ca_HepPh = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__ca_HepPh)
download_data_from_remote__ca_GrQc = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__ca_GrQc)
download_data_from_remote__web_BerkStan = ConstantTask(
    'DownloadDataFromRemote')
flow.add_node(download_data_from_remote__web_BerkStan)
download_data_from_remote__sentinel = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    download_data_from_remote__sentinel)
download_data_from_remote__web_NotreDame = ConstantTask(
    'DownloadDataFromRemote')
flow.add_node(download_data_from_remote__web_NotreDame)
download_data_from_remote__amazon0302 = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__amazon0302)
download_data_from_remote__web_Stanford = ConstantTask(
    'DownloadDataFromRemote')
flow.add_node(download_data_from_remote__web_Stanford)
download_data_from_remote__amazon0312 = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__amazon0312)
download_data_from_remote__ca_HepTh = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__ca_HepTh)
download_data_from_remote__web_Google = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_data_from_remote__web_Google)


decompress__amazon0505 = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/amazon0505/',
        file_name='amazon0505'))
flow.add_node(decompress__amazon0505)
flow.add_edge(download_data_from_remote__amazon0505, decompress__amazon0505)
decompress__amazon0601 = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='amazon0601',
        tmp_dir='/tmp/snap/amazon0601/'))
flow.add_node(decompress__amazon0601)
flow.add_edge(download_data_from_remote__amazon0601, decompress__amazon0601)
decompress__web_Stanford = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/web_Stanford/',
        file_name='web_Stanford'))
flow.add_node(decompress__web_Stanford)
flow.add_edge(
    download_data_from_remote__web_Stanford,
    decompress__web_Stanford)
decompress__web_Google = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='web_Google',
        tmp_dir='/tmp/snap/web_Google/'))
flow.add_node(decompress__web_Google)
flow.add_edge(download_data_from_remote__web_Google, decompress__web_Google)
decompress__sentinel = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/sentinel2',
        file_name='sentinel-2-metadata-table'))
flow.add_node(decompress__sentinel)
flow.add_edge(
    download_data_from_remote_gcs_location__sentinel_location,
    decompress__sentinel)
decompress__amazon0302 = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/amazon0302/',
        file_name='amazon0302'))
flow.add_node(decompress__amazon0302)
flow.add_edge(download_data_from_remote__amazon0302, decompress__amazon0302)
decompress__web_BerkStan = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/web_BerkStan/',
        file_name='web_BerkStan'))
flow.add_node(decompress__web_BerkStan)
flow.add_edge(
    download_data_from_remote__web_BerkStan,
    decompress__web_BerkStan)
decompress__amazon0312 = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='amazon0312',
        tmp_dir='/tmp/snap/amazon0312/'))
flow.add_node(decompress__amazon0312)
flow.add_edge(download_data_from_remote__amazon0312, decompress__amazon0312)
decompress__ca_AstroPh = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='ca_AstroPh',
        tmp_dir='/tmp/snap/ca_AstroPh/'))
flow.add_node(decompress__ca_AstroPh)
flow.add_edge(download_data_from_remote__ca_AstroPh, decompress__ca_AstroPh)
decompress__ca_HepTh = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='ca_HepTh',
        tmp_dir='/tmp/snap/ca_HepTh/'))
flow.add_node(decompress__ca_HepTh)
flow.add_edge(download_data_from_remote__ca_HepTh, decompress__ca_HepTh)
decompress__ca_CondMat = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/ca_CondMat/',
        file_name='ca_CondMat'))
flow.add_node(decompress__ca_CondMat)
flow.add_edge(download_data_from_remote__ca_CondMat, decompress__ca_CondMat)
decompress__ca_GrQc = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='ca_GrQc',
        tmp_dir='/tmp/snap/ca_GrQc/'))
flow.add_node(decompress__ca_GrQc)
flow.add_edge(download_data_from_remote__ca_GrQc, decompress__ca_GrQc)
decompress__web_NotreDame = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        file_name='web_NotreDame',
        tmp_dir='/tmp/snap/web_NotreDame/'))
flow.add_node(decompress__web_NotreDame)
flow.add_edge(
    download_data_from_remote__web_NotreDame,
    decompress__web_NotreDame)
decompress__ca_HepPh = ShellTask(
    command='gunzip {tmp_dir}/{file_name}'.format(
        tmp_dir='/tmp/snap/ca_HepPh/',
        file_name='ca_HepPh'))
flow.add_node(decompress__ca_HepPh)
flow.add_edge(download_data_from_remote__ca_HepPh, decompress__ca_HepPh)


remove_file_header__web_NotreDame = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='web_NotreDame', n='2', tmp_dir='/tmp/snap/web_NotreDame/'))
flow.add_node(remove_file_header__web_NotreDame)
flow.add_edge(decompress__web_NotreDame, remove_file_header__web_NotreDame)
remove_file_header__amazon0312 = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='amazon0312', tmp_dir='/tmp/snap/amazon0312/', n='2'))
flow.add_node(remove_file_header__amazon0312)
flow.add_edge(decompress__amazon0312, remove_file_header__amazon0312)
remove_file_header__web_Stanford = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', file_name='web_Stanford', tmp_dir='/tmp/snap/web_Stanford/'))
flow.add_node(remove_file_header__web_Stanford)
flow.add_edge(decompress__web_Stanford, remove_file_header__web_Stanford)
remove_file_header__ca_HepTh = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', file_name='ca_HepTh', tmp_dir='/tmp/snap/ca_HepTh/'))
flow.add_node(remove_file_header__ca_HepTh)
flow.add_edge(decompress__ca_HepTh, remove_file_header__ca_HepTh)
remove_file_header__ca_HepPh = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/snap/ca_HepPh/', file_name='ca_HepPh'))
flow.add_node(remove_file_header__ca_HepPh)
flow.add_edge(decompress__ca_HepPh, remove_file_header__ca_HepPh)
remove_file_header__ca_GrQc = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='ca_GrQc', tmp_dir='/tmp/snap/ca_GrQc/', n='2'))
flow.add_node(remove_file_header__ca_GrQc)
flow.add_edge(decompress__ca_GrQc, remove_file_header__ca_GrQc)
remove_file_header__amazon0505 = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/snap/amazon0505/', file_name='amazon0505'))
flow.add_node(remove_file_header__amazon0505)
flow.add_edge(decompress__amazon0505, remove_file_header__amazon0505)
remove_file_header__ca_CondMat = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='ca_CondMat', n='2', tmp_dir='/tmp/snap/ca_CondMat/'))
flow.add_node(remove_file_header__ca_CondMat)
flow.add_edge(decompress__ca_CondMat, remove_file_header__ca_CondMat)
remove_file_header__ca_AstroPh = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/snap/ca_AstroPh/', file_name='ca_AstroPh', n='2'))
flow.add_node(remove_file_header__ca_AstroPh)
flow.add_edge(decompress__ca_AstroPh, remove_file_header__ca_AstroPh)
remove_file_header__web_BerkStan = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(file_name='web_BerkStan', n='2', tmp_dir='/tmp/snap/web_BerkStan/'))
flow.add_node(remove_file_header__web_BerkStan)
flow.add_edge(decompress__web_BerkStan, remove_file_header__web_BerkStan)
remove_file_header__web_Google = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/snap/web_Google/', file_name='web_Google'))
flow.add_node(remove_file_header__web_Google)
flow.add_edge(decompress__web_Google, remove_file_header__web_Google)
remove_file_header__amazon0302 = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/snap/amazon0302/', n='2', file_name='amazon0302'))
flow.add_node(remove_file_header__amazon0302)
flow.add_edge(decompress__amazon0302, remove_file_header__amazon0302)
remove_file_header__amazon0601 = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/snap/amazon0601/', file_name='amazon0601', n='2'))
flow.add_node(remove_file_header__amazon0601)
flow.add_edge(decompress__amazon0601, remove_file_header__amazon0601)
remove_file_header__sentinel = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table', n='2'))
flow.add_node(remove_file_header__sentinel)
flow.add_edge(decompress__sentinel, remove_file_header__sentinel)


data_from_remote_downloaded__ca_CondMat = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__ca_CondMat)
for dep in [
        download_data_from_remote__ca_CondMat,
        remove_file_header__ca_CondMat]:
    flow.add_edge(dep, data_from_remote_downloaded__ca_CondMat)
data_from_remote_downloaded__web_Google = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__web_Google)
for dep in [
        download_data_from_remote__web_Google,
        remove_file_header__web_Google]:
    flow.add_edge(dep, data_from_remote_downloaded__web_Google)
data_from_remote_downloaded__amazon0312 = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__amazon0312)
for dep in [
        download_data_from_remote__amazon0312,
        remove_file_header__amazon0312]:
    flow.add_edge(dep, data_from_remote_downloaded__amazon0312)
data_from_remote_downloaded__amazon0505 = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__amazon0505)
for dep in [
        download_data_from_remote__amazon0505,
        remove_file_header__amazon0505]:
    flow.add_edge(dep, data_from_remote_downloaded__amazon0505)
data_from_remote_downloaded__web_Stanford = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__web_Stanford)
for dep in [
        download_data_from_remote__web_Stanford,
        remove_file_header__web_Stanford]:
    flow.add_edge(dep, data_from_remote_downloaded__web_Stanford)
data_from_remote_downloaded__amazon0302 = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__amazon0302)
for dep in [
        download_data_from_remote__amazon0302,
        remove_file_header__amazon0302]:
    flow.add_edge(dep, data_from_remote_downloaded__amazon0302)
data_from_remote_downloaded__ca_HepTh = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__ca_HepTh)
for dep in [download_data_from_remote__ca_HepTh, remove_file_header__ca_HepTh]:
    flow.add_edge(dep, data_from_remote_downloaded__ca_HepTh)
data_from_remote_downloaded__web_NotreDame = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__web_NotreDame)
for dep in [
        download_data_from_remote__web_NotreDame,
        remove_file_header__web_NotreDame]:
    flow.add_edge(dep, data_from_remote_downloaded__web_NotreDame)
data_from_remote_downloaded__ca_AstroPh = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__ca_AstroPh)
for dep in [
        download_data_from_remote__ca_AstroPh,
        remove_file_header__ca_AstroPh]:
    flow.add_edge(dep, data_from_remote_downloaded__ca_AstroPh)
data_from_remote_downloaded__ca_GrQc = ConstantTask('DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__ca_GrQc)
for dep in [download_data_from_remote__ca_GrQc, remove_file_header__ca_GrQc]:
    flow.add_edge(dep, data_from_remote_downloaded__ca_GrQc)
data_from_remote_downloaded__amazon0601 = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__amazon0601)
for dep in [
        download_data_from_remote__amazon0601,
        remove_file_header__amazon0601]:
    flow.add_edge(dep, data_from_remote_downloaded__amazon0601)
data_from_remote_downloaded__web_BerkStan = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__web_BerkStan)
for dep in [
        download_data_from_remote__web_BerkStan,
        remove_file_header__web_BerkStan]:
    flow.add_edge(dep, data_from_remote_downloaded__web_BerkStan)
data_from_remote_downloaded__ca_HepPh = ConstantTask(
    'DataFromRemoteDownloaded')
flow.add_node(data_from_remote_downloaded__ca_HepPh)
for dep in [download_data_from_remote__ca_HepPh, remove_file_header__ca_HepPh]:
    flow.add_edge(dep, data_from_remote_downloaded__ca_HepPh)


convert_csv_to_orc__ca_HepTh = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/snap/ca_HepTh/', table_name='ca_HepTh'))
flow.add_node(convert_csv_to_orc__ca_HepTh)
for dep in [download_data_from_remote__ca_HepTh, remove_file_header__ca_HepTh]:
    flow.add_edge(dep, convert_csv_to_orc__ca_HepTh)
convert_csv_to_orc__web_BerkStan = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/snap/web_BerkStan/', table_name='web_BerkStan'))
flow.add_node(convert_csv_to_orc__web_BerkStan)
for dep in [
        download_data_from_remote__web_BerkStan,
        remove_file_header__web_BerkStan]:
    flow.add_edge(dep, convert_csv_to_orc__web_BerkStan)
convert_csv_to_orc__amazon0601 = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/snap/amazon0601/', table_name='amazon0601', schema='from_id:BIGINT,to_id:BIGINT'))
flow.add_node(convert_csv_to_orc__amazon0601)
for dep in [
        download_data_from_remote__amazon0601,
        remove_file_header__amazon0601]:
    flow.add_edge(dep, convert_csv_to_orc__amazon0601)
convert_csv_to_orc__ca_AstroPh = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_AstroPh', tmp_dir='/tmp/snap/ca_AstroPh/'))
flow.add_node(convert_csv_to_orc__ca_AstroPh)
for dep in [
        download_data_from_remote__ca_AstroPh,
        remove_file_header__ca_AstroPh]:
    flow.add_edge(dep, convert_csv_to_orc__ca_AstroPh)
convert_csv_to_orc__web_NotreDame = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', table_name='web_NotreDame', tmp_dir='/tmp/snap/web_NotreDame/'))
flow.add_node(convert_csv_to_orc__web_NotreDame)
for dep in [
        download_data_from_remote__web_NotreDame,
        remove_file_header__web_NotreDame]:
    flow.add_edge(dep, convert_csv_to_orc__web_NotreDame)
convert_csv_to_orc__ca_CondMat = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(table_name='ca_CondMat', schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/snap/ca_CondMat/'))
flow.add_node(convert_csv_to_orc__ca_CondMat)
for dep in [
        download_data_from_remote__ca_CondMat,
        remove_file_header__ca_CondMat]:
    flow.add_edge(dep, convert_csv_to_orc__ca_CondMat)
convert_csv_to_orc__ca_GrQc = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_GrQc', tmp_dir='/tmp/snap/ca_GrQc/'))
flow.add_node(convert_csv_to_orc__ca_GrQc)
for dep in [download_data_from_remote__ca_GrQc, remove_file_header__ca_GrQc]:
    flow.add_edge(dep, convert_csv_to_orc__ca_GrQc)
convert_csv_to_orc__ca_HepPh = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/snap/ca_HepPh/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_HepPh'))
flow.add_node(convert_csv_to_orc__ca_HepPh)
for dep in [download_data_from_remote__ca_HepPh, remove_file_header__ca_HepPh]:
    flow.add_edge(dep, convert_csv_to_orc__ca_HepPh)
convert_csv_to_orc__amazon0312 = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(tmp_dir='/tmp/snap/amazon0312/', schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0312'))
flow.add_node(convert_csv_to_orc__amazon0312)
for dep in [
        download_data_from_remote__amazon0312,
        remove_file_header__amazon0312]:
    flow.add_edge(dep, convert_csv_to_orc__amazon0312)
convert_csv_to_orc__sentinel = ShellTask(
    command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(
        table_name='sentinel-2-metadata-table',
        schema=(
            'granule_id:STRING,product_id:STRING'
            ',datatake_identifier:STRING,mgrs_tile:STRING'
            ',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
            ',geometric_quality_flag:STRING,generation_time:BIGINT'
            ',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT'),
        tmp_dir='/tmp/sentinel2'))
flow.add_node(convert_csv_to_orc__sentinel)
for dep in [download_data_from_remote__sentinel, remove_file_header__sentinel]:
    flow.add_edge(dep, convert_csv_to_orc__sentinel)
convert_csv_to_orc__web_Stanford = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/snap/web_Stanford/', table_name='web_Stanford'))
flow.add_node(convert_csv_to_orc__web_Stanford)
for dep in [
        download_data_from_remote__web_Stanford,
        remove_file_header__web_Stanford]:
    flow.add_edge(dep, convert_csv_to_orc__web_Stanford)
convert_csv_to_orc__web_Google = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', tmp_dir='/tmp/snap/web_Google/', table_name='web_Google'))
flow.add_node(convert_csv_to_orc__web_Google)
for dep in [
        download_data_from_remote__web_Google,
        remove_file_header__web_Google]:
    flow.add_edge(dep, convert_csv_to_orc__web_Google)
convert_csv_to_orc__amazon0505 = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0505', tmp_dir='/tmp/snap/amazon0505/'))
flow.add_node(convert_csv_to_orc__amazon0505)
for dep in [
        download_data_from_remote__amazon0505,
        remove_file_header__amazon0505]:
    flow.add_edge(dep, convert_csv_to_orc__amazon0505)
convert_csv_to_orc__amazon0302 = ShellTask(command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(table_name='amazon0302', tmp_dir='/tmp/snap/amazon0302/', schema='from_id:BIGINT,to_id:BIGINT'))
flow.add_node(convert_csv_to_orc__amazon0302)
for dep in [
        download_data_from_remote__amazon0302,
        remove_file_header__amazon0302]:
    flow.add_edge(dep, convert_csv_to_orc__amazon0302)


upload_data_to_local__ca_GrQc = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__ca_GrQc)
flow.add_edge(convert_csv_to_orc__ca_GrQc, upload_data_to_local__ca_GrQc)
upload_data_to_local__amazon0302 = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__amazon0302)
flow.add_edge(convert_csv_to_orc__amazon0302, upload_data_to_local__amazon0302)
upload_data_to_local__web_Stanford = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__web_Stanford)
flow.add_edge(convert_csv_to_orc__web_Stanford,
              upload_data_to_local__web_Stanford)
upload_data_to_local__amazon0312 = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__amazon0312)
flow.add_edge(convert_csv_to_orc__amazon0312, upload_data_to_local__amazon0312)
upload_data_to_local__amazon0505 = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__amazon0505)
flow.add_edge(convert_csv_to_orc__amazon0505, upload_data_to_local__amazon0505)
upload_data_to_local__amazon0601 = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__amazon0601)
flow.add_edge(convert_csv_to_orc__amazon0601, upload_data_to_local__amazon0601)
upload_data_to_local__ca_HepTh = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__ca_HepTh)
flow.add_edge(convert_csv_to_orc__ca_HepTh, upload_data_to_local__ca_HepTh)
upload_data_to_local__web_NotreDame = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__web_NotreDame)
flow.add_edge(convert_csv_to_orc__web_NotreDame,
              upload_data_to_local__web_NotreDame)
upload_data_to_local__web_BerkStan = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__web_BerkStan)
flow.add_edge(convert_csv_to_orc__web_BerkStan,
              upload_data_to_local__web_BerkStan)
upload_data_to_local__ca_CondMat = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__ca_CondMat)
flow.add_edge(convert_csv_to_orc__ca_CondMat, upload_data_to_local__ca_CondMat)
upload_data_to_local__web_Google = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__web_Google)
flow.add_edge(convert_csv_to_orc__web_Google, upload_data_to_local__web_Google)
upload_data_to_local__ca_AstroPh = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__ca_AstroPh)
flow.add_edge(convert_csv_to_orc__ca_AstroPh, upload_data_to_local__ca_AstroPh)
upload_data_to_local__ca_HepPh = ConstantTask('UploadDataToLocal')
flow.add_node(upload_data_to_local__ca_HepPh)
flow.add_edge(convert_csv_to_orc__ca_HepPh, upload_data_to_local__ca_HepPh)


replicated_data__sentinel = ConstantTask('ReplicatedData')
flow.add_node(replicated_data__sentinel)
for dep in [
        download_data_from_remote_gcs_location__sentinel_location,
        download_data_from_remote__sentinel,
        remove_file_header__sentinel,
        convert_csv_to_orc__sentinel]:
    flow.add_edge(dep, replicated_data__sentinel)


replicated_assets__web_NotreDame = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__web_NotreDame)
for dep in [replicated_schema__web_NotreDame,
            download_data_from_remote__web_NotreDame,
            data_from_remote_downloaded__web_NotreDame,
            upload_data_to_local__web_NotreDame]:
    flow.add_edge(dep, replicated_assets__web_NotreDame)
replicated_assets__ca_CondMat = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__ca_CondMat)
for dep in [replicated_schema__ca_CondMat,
            download_data_from_remote__ca_CondMat,
            data_from_remote_downloaded__ca_CondMat,
            upload_data_to_local__ca_CondMat]:
    flow.add_edge(dep, replicated_assets__ca_CondMat)
replicated_assets__amazon0302 = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__amazon0302)
for dep in [replicated_schema__amazon0302,
            download_data_from_remote__amazon0302,
            data_from_remote_downloaded__amazon0302,
            upload_data_to_local__amazon0302]:
    flow.add_edge(dep, replicated_assets__amazon0302)
replicated_assets__web_BerkStan = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__web_BerkStan)
for dep in [replicated_schema__web_BerkStan,
            download_data_from_remote__web_BerkStan,
            data_from_remote_downloaded__web_BerkStan,
            upload_data_to_local__web_BerkStan]:
    flow.add_edge(dep, replicated_assets__web_BerkStan)
replicated_assets__web_Google = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__web_Google)
for dep in [replicated_schema__web_Google,
            download_data_from_remote__web_Google,
            data_from_remote_downloaded__web_Google,
            upload_data_to_local__web_Google]:
    flow.add_edge(dep, replicated_assets__web_Google)
replicated_assets__amazon0505 = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__amazon0505)
for dep in [replicated_schema__amazon0505,
            download_data_from_remote__amazon0505,
            data_from_remote_downloaded__amazon0505,
            upload_data_to_local__amazon0505]:
    flow.add_edge(dep, replicated_assets__amazon0505)
replicated_assets__web_Stanford = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__web_Stanford)
for dep in [replicated_schema__web_Stanford,
            download_data_from_remote__web_Stanford,
            data_from_remote_downloaded__web_Stanford,
            upload_data_to_local__web_Stanford]:
    flow.add_edge(dep, replicated_assets__web_Stanford)
replicated_assets__ca_GrQc = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__ca_GrQc)
for dep in [
        replicated_schema__ca_GrQc,
        download_data_from_remote__ca_GrQc,
        data_from_remote_downloaded__ca_GrQc,
        upload_data_to_local__ca_GrQc]:
    flow.add_edge(dep, replicated_assets__ca_GrQc)
replicated_assets__ca_HepTh = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__ca_HepTh)
for dep in [
        replicated_schema__ca_HepTh,
        download_data_from_remote__ca_HepTh,
        data_from_remote_downloaded__ca_HepTh,
        upload_data_to_local__ca_HepTh]:
    flow.add_edge(dep, replicated_assets__ca_HepTh)
replicated_assets__ca_HepPh = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__ca_HepPh)
for dep in [
        replicated_schema__ca_HepPh,
        download_data_from_remote__ca_HepPh,
        data_from_remote_downloaded__ca_HepPh,
        upload_data_to_local__ca_HepPh]:
    flow.add_edge(dep, replicated_assets__ca_HepPh)
replicated_assets__amazon0312 = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__amazon0312)
for dep in [replicated_schema__amazon0312,
            download_data_from_remote__amazon0312,
            data_from_remote_downloaded__amazon0312,
            upload_data_to_local__amazon0312]:
    flow.add_edge(dep, replicated_assets__amazon0312)
replicated_assets__amazon0601 = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__amazon0601)
for dep in [replicated_schema__amazon0601,
            download_data_from_remote__amazon0601,
            data_from_remote_downloaded__amazon0601,
            upload_data_to_local__amazon0601]:
    flow.add_edge(dep, replicated_assets__amazon0601)
replicated_assets__ca_AstroPh = ConstantTask('ReplicatedAssets')
flow.add_node(replicated_assets__ca_AstroPh)
for dep in [replicated_schema__ca_AstroPh,
            download_data_from_remote__ca_AstroPh,
            data_from_remote_downloaded__ca_AstroPh,
            upload_data_to_local__ca_AstroPh]:
    flow.add_edge(dep, replicated_assets__ca_AstroPh)


replicated_data_sets__sentinel = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__sentinel)
for dep in [replicated_schema__sentinel, replicated_data__sentinel]:
    flow.add_edge(dep, replicated_data_sets__sentinel)
replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [
        replicated_assets__web_NotreDame,
        replicated_assets__ca_CondMat,
        replicated_assets__amazon0302,
        replicated_assets__web_BerkStan,
        replicated_assets__web_Google,
        replicated_assets__amazon0505,
        replicated_assets__web_Stanford,
        replicated_assets__ca_GrQc,
        replicated_assets__ca_HepTh,
        replicated_assets__ca_HepPh,
        replicated_assets__amazon0312,
        replicated_assets__amazon0601,
        replicated_assets__ca_AstroPh]:
    flow.add_edge(dep, replicated_data_sets__snap)


is_consistent__basic_data_setup = ConstantTask('IsConsistent')
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_consistent__basic_data_setup)


is_audited_table__amazon0601 = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__amazon0601)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__amazon0601)
is_audited_table__sentinel = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__sentinel)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__sentinel)
is_audited_table__web_BerkStan = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__web_BerkStan)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__web_BerkStan)
is_audited_table__web_Stanford = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__web_Stanford)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__web_Stanford)
is_audited_table__ca_CondMat = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__ca_CondMat)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__ca_CondMat)
is_audited_table__amazon0312 = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__amazon0312)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__amazon0312)
is_audited_table__ca_HepPh = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__ca_HepPh)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__ca_HepPh)
is_audited_table__amazon0302 = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__amazon0302)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__amazon0302)
is_audited_table__ca_GrQc = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__ca_GrQc)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__ca_GrQc)
is_audited_table__ca_AstroPh = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__ca_AstroPh)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__ca_AstroPh)
is_audited_table__ca_HepTh = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__ca_HepTh)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__ca_HepTh)
is_audited_table__amazon0505 = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__amazon0505)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__amazon0505)
is_audited_table__web_Google = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__web_Google)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__web_Google)
is_audited_table__web_NotreDame = ConstantTask('IsAuditedTable')
flow.add_node(is_audited_table__web_NotreDame)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(dep, is_audited_table__web_NotreDame)


is_audited__basic_data_setup = ConstantTask('IsAudited')
flow.add_node(is_audited__basic_data_setup)
for dep in [
        is_audited_table__amazon0601,
        is_audited_table__sentinel,
        is_audited_table__web_BerkStan,
        is_audited_table__web_Stanford,
        is_audited_table__ca_CondMat,
        is_audited_table__amazon0312,
        is_audited_table__ca_HepPh,
        is_audited_table__amazon0302,
        is_audited_table__ca_GrQc,
        is_audited_table__ca_AstroPh,
        is_audited_table__ca_HepTh,
        is_audited_table__amazon0505,
        is_audited_table__web_Google,
        is_audited_table__web_NotreDame]:
    flow.add_edge(dep, is_audited__basic_data_setup)
