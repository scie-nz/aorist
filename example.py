from google.cloud import storage
def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_name):
  client = storage.Client.from_service_account_json('/gcloud/social_norms.json')
  bucket = client.bucket(bucket_name)
  blob = bucket.blob(blob_name)
  dest = "%s/%s" % (tmp_dir, file_name)
  blob.download_to_filename(dest)


hive_schemas_created_schema = """
    from_id BIGINT,
    to_id BIGINT
"""
hive_schemas_created_command = """
presto -e 'CREATE TABLE IF NOT EXISTS {table_name} (
{schema} ) WITH (format='{data_format}');
'"""
params_hive_schemas_created = {'web_NotreDame': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='web_NotreDame', data_format='ORC', schema=hive_schemas_created_schema)},
'ca_GrQc': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='ca_GrQc', data_format='ORC', schema=hive_schemas_created_schema)},
'web_Google': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='web_Google', data_format='ORC', schema=hive_schemas_created_schema)},
'sentinel': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='sentinel-2-metadata-table', data_format='ORC', schema="""
    granule_id VARCHAR,
    product_id VARCHAR,
    datatake_identifier VARCHAR,
    mgrs_tile VARCHAR,
    sensing_time BIGINT,
    total_size BIGINT,
    cloud_cover VARCHAR,
    geometric_quality_flag VARCHAR,
    generation_time BIGINT,
    north_lat REAL COMMENT 'Northern latitude of the tile`s bounding box.',
    south_lat REAL COMMENT 'Southern latitude of the tile`s bounding box.',
    west_lon REAL COMMENT 'Western longitude of the tile`s bounding box.',
    east_lon REAL COMMENT 'Eastern longitude of the tile`s bounding box.',
    base_url VARCHAR
""")},
'ca_AstroPh': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='ca_AstroPh', data_format='ORC', schema=hive_schemas_created_schema)},
'ca_HepTh': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='ca_HepTh', data_format='ORC', schema=hive_schemas_created_schema)},
'web_Stanford': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='web_Stanford', data_format='ORC', schema=hive_schemas_created_schema)},
'amazon0601': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='amazon0601', data_format='ORC', schema=hive_schemas_created_schema)},
'ca_HepPh': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='ca_HepPh', data_format='ORC', schema=hive_schemas_created_schema)},
'amazon0312': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='amazon0312', data_format='ORC', schema=hive_schemas_created_schema)},
'amazon0302': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='amazon0302', data_format='ORC', schema=hive_schemas_created_schema)},
'web_BerkStan': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='web_BerkStan', data_format='ORC', schema=hive_schemas_created_schema)},
'ca_CondMat': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='ca_CondMat', data_format='ORC', schema=hive_schemas_created_schema)},
'amazon0505': {'dep_list': [],
'command': hive_schemas_created_command.format(table_name='amazon0505', data_format='ORC', schema=hive_schemas_created_schema)}}
for (t, params) in params_hive_schemas_created.items():
    tasks_hive_schemas_created[t] = ShellTask(command=params_hive_schemas_created['command'])
    flow.add_node(tasks_hive_schemas_created[t])
    for dep in params_hive_schemas_created['dep_list']:
        flow.add_edge(tasks_hive_schemas_created[t], dep)


params_download_data_from_remote_web_location = {'web_Stanford': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_Stanford', address='https://snap.stanford.edu/data/web-Stanford.txt.gz', tmp_dir='/tmp/snap/web_Stanford/')},
'web_Google': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_Google', address='https://snap.stanford.edu/data/web-Google.txt.gz', tmp_dir='/tmp/snap/web_Google/')},
'ca_CondMat': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_CondMat', address='https://snap.stanford.edu/data/ca-CondMat.txt.gz', tmp_dir='/tmp/snap/ca_CondMat/')},
'ca_HepTh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_HepTh', address='https://snap.stanford.edu/data/ca-HepTh.txt.gz', tmp_dir='/tmp/snap/ca_HepTh/')},
'amazon0302': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0302', address='https://snap.stanford.edu/data/amazon0302.txt.gz', tmp_dir='/tmp/snap/amazon0302/')},
'ca_HepPh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_HepPh', address='https://snap.stanford.edu/data/ca-HepPh.txt.gz', tmp_dir='/tmp/snap/ca_HepPh/')},
'web_BerkStan': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_BerkStan', address='https://snap.stanford.edu/data/web-BerkStan.txt.gz', tmp_dir='/tmp/snap/web_BerkStan/')},
'web_NotreDame': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_NotreDame', address='https://snap.stanford.edu/data/web-NotreDame.txt.gz', tmp_dir='/tmp/snap/web_NotreDame/')},
'amazon0312': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0312', address='https://snap.stanford.edu/data/amazon0312.txt.gz', tmp_dir='/tmp/snap/amazon0312/')},
'amazon0505': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0505', address='https://snap.stanford.edu/data/amazon0505.txt.gz', tmp_dir='/tmp/snap/amazon0505/')},
'ca_AstroPh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_AstroPh', address='https://snap.stanford.edu/data/ca-AstroPh.txt.gz', tmp_dir='/tmp/snap/ca_AstroPh/')},
'amazon0601': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0601', address='https://snap.stanford.edu/data/amazon0601.txt.gz', tmp_dir='/tmp/snap/amazon0601/')},
'ca_GrQc': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_GrQc', address='https://snap.stanford.edu/data/ca-GrQc.txt.gz', tmp_dir='/tmp/snap/ca_GrQc/')}}
for (t, params) in params_download_data_from_remote_web_location.items():
    tasks_download_data_from_remote_web_location[t] = ShellTask(command=params_download_data_from_remote_web_location['command'])
    flow.add_node(tasks_download_data_from_remote_web_location[t])
    for dep in params_download_data_from_remote_web_location['dep_list']:
        flow.add_edge(tasks_download_data_from_remote_web_location[t], dep)


params_replicated_schema = {'sentinel': {'dep_list': [tasks_hive_schemas_created['sentinel']]},
'amazon0312': {'dep_list': [tasks_hive_schemas_created['amazon0312']]},
'web_Stanford': {'dep_list': [tasks_hive_schemas_created['web_Stanford']]},
'ca_AstroPh': {'dep_list': [tasks_hive_schemas_created['ca_AstroPh']]},
'web_BerkStan': {'dep_list': [tasks_hive_schemas_created['web_BerkStan']]},
'web_NotreDame': {'dep_list': [tasks_hive_schemas_created['web_NotreDame']]},
'web_Google': {'dep_list': [tasks_hive_schemas_created['web_Google']]},
'ca_HepPh': {'dep_list': [tasks_hive_schemas_created['ca_HepPh']]},
'ca_GrQc': {'dep_list': [tasks_hive_schemas_created['ca_GrQc']]},
'ca_CondMat': {'dep_list': [tasks_hive_schemas_created['ca_CondMat']]},
'amazon0302': {'dep_list': [tasks_hive_schemas_created['amazon0302']]},
'ca_HepTh': {'dep_list': [tasks_hive_schemas_created['ca_HepTh']]},
'amazon0601': {'dep_list': [tasks_hive_schemas_created['amazon0601']]},
'amazon0505': {'dep_list': [tasks_hive_schemas_created['amazon0505']]}}
for (t, params) in params_replicated_schema.items():
    tasks_replicated_schema[t] = ConstantTask('ReplicatedSchema')
    flow.add_node(tasks_replicated_schema[t])
    for dep in params_replicated_schema['dep_list']:
        flow.add_edge(tasks_replicated_schema[t], dep)


download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)



params_decompress = {'sentinel': {'dep_list': [download_data_from_remote_gcs_location__sentinel_location],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table')},
'ca_HepPh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_HepPh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_HepPh/', file_name='ca_HepPh')},
'ca_HepTh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_HepTh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_HepTh/', file_name='ca_HepTh')},
'web_Stanford': {'dep_list': [tasks_download_data_from_remote_web_location['web_Stanford']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_Stanford/', file_name='web_Stanford')},
'ca_AstroPh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_AstroPh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_AstroPh/', file_name='ca_AstroPh')},
'amazon0505': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0505']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0505/', file_name='amazon0505')},
'ca_CondMat': {'dep_list': [tasks_download_data_from_remote_web_location['ca_CondMat']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_CondMat/', file_name='ca_CondMat')},
'amazon0312': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0312']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0312/', file_name='amazon0312')},
'web_NotreDame': {'dep_list': [tasks_download_data_from_remote_web_location['web_NotreDame']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_NotreDame/', file_name='web_NotreDame')},
'amazon0302': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0302']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0302/', file_name='amazon0302')},
'web_Google': {'dep_list': [tasks_download_data_from_remote_web_location['web_Google']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_Google/', file_name='web_Google')},
'web_BerkStan': {'dep_list': [tasks_download_data_from_remote_web_location['web_BerkStan']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_BerkStan/', file_name='web_BerkStan')},
'amazon0601': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0601']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0601/', file_name='amazon0601')},
'ca_GrQc': {'dep_list': [tasks_download_data_from_remote_web_location['ca_GrQc']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_GrQc/', file_name='ca_GrQc')}}
for (t, params) in params_decompress.items():
    tasks_decompress[t] = ShellTask(command=params_decompress['command'])
    flow.add_node(tasks_decompress[t])
    for dep in params_decompress['dep_list']:
        flow.add_edge(tasks_decompress[t], dep)



remove_file_header_command = """
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
"""
params_remove_file_header = {'web_Stanford': {'dep_list': [tasks_decompress['web_Stanford']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/web_Stanford/', file_name='web_Stanford', n='2')},
'ca_HepPh': {'dep_list': [tasks_decompress['ca_HepPh']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/ca_HepPh/', file_name='ca_HepPh', n='2')},
'amazon0302': {'dep_list': [tasks_decompress['amazon0302']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/amazon0302/', file_name='amazon0302', n='2')},
'ca_AstroPh': {'dep_list': [tasks_decompress['ca_AstroPh']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/ca_AstroPh/', file_name='ca_AstroPh', n='2')},
'amazon0601': {'dep_list': [tasks_decompress['amazon0601']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/amazon0601/', file_name='amazon0601', n='2')},
'ca_GrQc': {'dep_list': [tasks_decompress['ca_GrQc']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/ca_GrQc/', file_name='ca_GrQc', n='2')},
'ca_CondMat': {'dep_list': [tasks_decompress['ca_CondMat']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/ca_CondMat/', file_name='ca_CondMat', n='2')},
'amazon0312': {'dep_list': [tasks_decompress['amazon0312']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/amazon0312/', file_name='amazon0312', n='2')},
'ca_HepTh': {'dep_list': [tasks_decompress['ca_HepTh']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/ca_HepTh/', file_name='ca_HepTh', n='2')},
'sentinel': {'dep_list': [tasks_decompress['sentinel']],
'command': remove_file_header_command.format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table', n='2')},
'amazon0505': {'dep_list': [tasks_decompress['amazon0505']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/amazon0505/', file_name='amazon0505', n='2')},
'web_NotreDame': {'dep_list': [tasks_decompress['web_NotreDame']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/web_NotreDame/', file_name='web_NotreDame', n='2')},
'web_BerkStan': {'dep_list': [tasks_decompress['web_BerkStan']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/web_BerkStan/', file_name='web_BerkStan', n='2')},
'web_Google': {'dep_list': [tasks_decompress['web_Google']],
'command': remove_file_header_command.format(tmp_dir='/tmp/snap/web_Google/', file_name='web_Google', n='2')}}
for (t, params) in params_remove_file_header.items():
    tasks_remove_file_header[t] = ShellTask(command=params_remove_file_header['command'])
    flow.add_node(tasks_remove_file_header[t])
    for dep in params_remove_file_header['dep_list']:
        flow.add_edge(tasks_remove_file_header[t], dep)


params_file_ready_for_upload = {'web_NotreDame': {'dep_list': [tasks_remove_file_header['web_NotreDame']]},
'amazon0505': {'dep_list': [tasks_remove_file_header['amazon0505']]},
'ca_HepPh': {'dep_list': [tasks_remove_file_header['ca_HepPh']]},
'web_BerkStan': {'dep_list': [tasks_remove_file_header['web_BerkStan']]},
'amazon0302': {'dep_list': [tasks_remove_file_header['amazon0302']]},
'ca_CondMat': {'dep_list': [tasks_remove_file_header['ca_CondMat']]},
'amazon0312': {'dep_list': [tasks_remove_file_header['amazon0312']]},
'amazon0601': {'dep_list': [tasks_remove_file_header['amazon0601']]},
'ca_AstroPh': {'dep_list': [tasks_remove_file_header['ca_AstroPh']]},
'ca_GrQc': {'dep_list': [tasks_remove_file_header['ca_GrQc']]},
'ca_HepTh': {'dep_list': [tasks_remove_file_header['ca_HepTh']]},
'web_Google': {'dep_list': [tasks_remove_file_header['web_Google']]},
'web_Stanford': {'dep_list': [tasks_remove_file_header['web_Stanford']]}}
for (t, params) in params_file_ready_for_upload.items():
    tasks_file_ready_for_upload[t] = ConstantTask('FileReadyForUpload')
    flow.add_node(tasks_file_ready_for_upload[t])
    for dep in params_file_ready_for_upload['dep_list']:
        flow.add_edge(tasks_file_ready_for_upload[t], dep)


convert_csv_to_orc_command = """
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
"""
params_convert_csv_to_orc = {'ca_AstroPh': {'dep_list': [tasks_remove_file_header['ca_AstroPh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_AstroPh/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_AstroPh')},
'amazon0312': {'dep_list': [tasks_remove_file_header['amazon0312']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0312/', schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0312')},
'web_NotreDame': {'dep_list': [tasks_remove_file_header['web_NotreDame']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_NotreDame/', schema='from_id:BIGINT,to_id:BIGINT', table_name='web_NotreDame')},
'ca_CondMat': {'dep_list': [tasks_remove_file_header['ca_CondMat']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_CondMat/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_CondMat')},
'web_Stanford': {'dep_list': [tasks_remove_file_header['web_Stanford']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_Stanford/', schema='from_id:BIGINT,to_id:BIGINT', table_name='web_Stanford')},
'amazon0505': {'dep_list': [tasks_remove_file_header['amazon0505']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0505/', schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0505')},
'web_Google': {'dep_list': [tasks_remove_file_header['web_Google']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_Google/', schema='from_id:BIGINT,to_id:BIGINT', table_name='web_Google')},
'amazon0302': {'dep_list': [tasks_remove_file_header['amazon0302']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0302/', schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0302')},
'amazon0601': {'dep_list': [tasks_remove_file_header['amazon0601']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0601/', schema='from_id:BIGINT,to_id:BIGINT', table_name='amazon0601')},
'web_BerkStan': {'dep_list': [tasks_remove_file_header['web_BerkStan']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_BerkStan/', schema='from_id:BIGINT,to_id:BIGINT', table_name='web_BerkStan')},
'ca_HepPh': {'dep_list': [tasks_remove_file_header['ca_HepPh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_HepPh/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_HepPh')},
'ca_HepTh': {'dep_list': [tasks_remove_file_header['ca_HepTh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_HepTh/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_HepTh')},
'ca_GrQc': {'dep_list': [tasks_remove_file_header['ca_GrQc']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_GrQc/', schema='from_id:BIGINT,to_id:BIGINT', table_name='ca_GrQc')},
'sentinel': {'dep_list': [tasks_remove_file_header['sentinel']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/sentinel2', schema=('granule_id:STRING,product_id:STRING'
',datatake_identifier:STRING,mgrs_tile:STRING'
',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
',geometric_quality_flag:STRING,generation_time:BIGINT'
',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT'
',east_lon:FLOAT,base_url:STRING'), table_name='sentinel-2-metadata-table')}}
for (t, params) in params_convert_csv_to_orc.items():
    tasks_convert_csv_to_orc[t] = ShellTask(command=params_convert_csv_to_orc['command'])
    flow.add_node(tasks_convert_csv_to_orc[t])
    for dep in params_convert_csv_to_orc['dep_list']:
        flow.add_edge(tasks_convert_csv_to_orc[t], dep)


params_upload_data_to_local = {'amazon0601': {'dep_list': [tasks_convert_csv_to_orc['amazon0601']]},
'amazon0505': {'dep_list': [tasks_convert_csv_to_orc['amazon0505']]},
'web_Google': {'dep_list': [tasks_convert_csv_to_orc['web_Google']]},
'ca_HepTh': {'dep_list': [tasks_convert_csv_to_orc['ca_HepTh']]},
'ca_AstroPh': {'dep_list': [tasks_convert_csv_to_orc['ca_AstroPh']]},
'ca_GrQc': {'dep_list': [tasks_convert_csv_to_orc['ca_GrQc']]},
'amazon0302': {'dep_list': [tasks_convert_csv_to_orc['amazon0302']]},
'ca_CondMat': {'dep_list': [tasks_convert_csv_to_orc['ca_CondMat']]},
'web_Stanford': {'dep_list': [tasks_convert_csv_to_orc['web_Stanford']]},
'amazon0312': {'dep_list': [tasks_convert_csv_to_orc['amazon0312']]},
'web_BerkStan': {'dep_list': [tasks_convert_csv_to_orc['web_BerkStan']]},
'ca_HepPh': {'dep_list': [tasks_convert_csv_to_orc['ca_HepPh']]},
'web_NotreDame': {'dep_list': [tasks_convert_csv_to_orc['web_NotreDame']]}}
for (t, params) in params_upload_data_to_local.items():
    tasks_upload_data_to_local[t] = ConstantTask('UploadDataToLocal')
    flow.add_node(tasks_upload_data_to_local[t])
    for dep in params_upload_data_to_local['dep_list']:
        flow.add_edge(tasks_upload_data_to_local[t], dep)


replicated_data__sentinel = ConstantTask('ReplicatedData')
flow.add_node(replicated_data__sentinel)
for dep in [tasks_remove_file_header['sentinel'], tasks_convert_csv_to_orc['sentinel']]:
    flow.add_edge(replicated_data__sentinel, dep)


params_replicated_assets = {'web_Google': {'dep_list': [tasks_replicated_schema['web_Google'], tasks_file_ready_for_upload['web_Google'], tasks_upload_data_to_local['web_Google']]},
'ca_HepTh': {'dep_list': [tasks_replicated_schema['ca_HepTh'], tasks_file_ready_for_upload['ca_HepTh'], tasks_upload_data_to_local['ca_HepTh']]},
'web_NotreDame': {'dep_list': [tasks_replicated_schema['web_NotreDame'], tasks_file_ready_for_upload['web_NotreDame'], tasks_upload_data_to_local['web_NotreDame']]},
'amazon0302': {'dep_list': [tasks_replicated_schema['amazon0302'], tasks_file_ready_for_upload['amazon0302'], tasks_upload_data_to_local['amazon0302']]},
'amazon0505': {'dep_list': [tasks_replicated_schema['amazon0505'], tasks_file_ready_for_upload['amazon0505'], tasks_upload_data_to_local['amazon0505']]},
'amazon0601': {'dep_list': [tasks_replicated_schema['amazon0601'], tasks_file_ready_for_upload['amazon0601'], tasks_upload_data_to_local['amazon0601']]},
'amazon0312': {'dep_list': [tasks_replicated_schema['amazon0312'], tasks_file_ready_for_upload['amazon0312'], tasks_upload_data_to_local['amazon0312']]},
'ca_AstroPh': {'dep_list': [tasks_replicated_schema['ca_AstroPh'], tasks_file_ready_for_upload['ca_AstroPh'], tasks_upload_data_to_local['ca_AstroPh']]},
'ca_HepPh': {'dep_list': [tasks_replicated_schema['ca_HepPh'], tasks_file_ready_for_upload['ca_HepPh'], tasks_upload_data_to_local['ca_HepPh']]},
'ca_CondMat': {'dep_list': [tasks_replicated_schema['ca_CondMat'], tasks_file_ready_for_upload['ca_CondMat'], tasks_upload_data_to_local['ca_CondMat']]},
'ca_GrQc': {'dep_list': [tasks_replicated_schema['ca_GrQc'], tasks_file_ready_for_upload['ca_GrQc'], tasks_upload_data_to_local['ca_GrQc']]},
'web_BerkStan': {'dep_list': [tasks_replicated_schema['web_BerkStan'], tasks_file_ready_for_upload['web_BerkStan'], tasks_upload_data_to_local['web_BerkStan']]},
'web_Stanford': {'dep_list': [tasks_replicated_schema['web_Stanford'], tasks_file_ready_for_upload['web_Stanford'], tasks_upload_data_to_local['web_Stanford']]}}
for (t, params) in params_replicated_assets.items():
    tasks_replicated_assets[t] = ConstantTask('ReplicatedAssets')
    flow.add_node(tasks_replicated_assets[t])
    for dep in params_replicated_assets['dep_list']:
        flow.add_edge(tasks_replicated_assets[t], dep)


replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [tasks_replicated_assets['web_Stanford'], tasks_replicated_assets['amazon0505'], tasks_replicated_assets['amazon0601'], tasks_replicated_assets['ca_GrQc'], tasks_replicated_assets['ca_AstroPh'], tasks_replicated_assets['amazon0312'], tasks_replicated_assets['web_Google'], tasks_replicated_assets['ca_CondMat'], tasks_replicated_assets['ca_HepPh'], tasks_replicated_assets['ca_HepTh'], tasks_replicated_assets['web_BerkStan'], tasks_replicated_assets['web_NotreDame'], tasks_replicated_assets['amazon0302']]:
    flow.add_edge(replicated_data_sets__snap, dep)

replicated_data_sets__sentinel = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__sentinel)
for dep in [tasks_replicated_schema['sentinel'], replicated_data__sentinel]:
    flow.add_edge(replicated_data_sets__sentinel, dep)


is_consistent__basic_data_setup = ConstantTask('IsConsistent')
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__snap, replicated_data_sets__sentinel]:
    flow.add_edge(is_consistent__basic_data_setup, dep)


params_is_audited_table = {'ca_HepPh': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'amazon0601': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'amazon0302': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'ca_HepTh': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'ca_GrQc': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'web_NotreDame': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'ca_AstroPh': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'web_Stanford': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'amazon0312': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'amazon0505': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'web_Google': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'sentinel': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'ca_CondMat': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]},
'web_BerkStan': {'dep_list': [replicated_data_sets__snap, replicated_data_sets__sentinel]}}
for (t, params) in params_is_audited_table.items():
    tasks_is_audited_table[t] = ConstantTask('IsAuditedTable')
    flow.add_node(tasks_is_audited_table[t])
    for dep in params_is_audited_table['dep_list']:
        flow.add_edge(tasks_is_audited_table[t], dep)


is_audited__basic_data_setup = ConstantTask('IsAudited')
flow.add_node(is_audited__basic_data_setup)
for dep in [tasks_is_audited_table['ca_HepTh'], tasks_is_audited_table['amazon0312'], tasks_is_audited_table['ca_AstroPh'], tasks_is_audited_table['web_Stanford'], tasks_is_audited_table['ca_GrQc'], tasks_is_audited_table['amazon0601'], tasks_is_audited_table['ca_HepPh'], tasks_is_audited_table['web_BerkStan'], tasks_is_audited_table['amazon0302'], tasks_is_audited_table['sentinel'], tasks_is_audited_table['web_Google'], tasks_is_audited_table['web_NotreDame'], tasks_is_audited_table['amazon0505'], tasks_is_audited_table['ca_CondMat']]:
    flow.add_edge(is_audited__basic_data_setup, dep)


