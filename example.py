from google.cloud import storage
def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_name):
  client = storage.Client.from_service_account_json('/gcloud/social_norms.json')
  bucket = client.bucket(bucket_name)
  blob = bucket.blob(blob_name)
  dest = "%s/%s" % (tmp_dir, file_name)
  blob.download_to_filename(dest)


params_download_data_from_remote_web_location = {'ca_CondMat': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_CondMat', tmp_dir='/tmp/snap/ca_CondMat/', address='https://snap.stanford.edu/data/ca-CondMat.txt.gz')},
'amazon0601': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0601', tmp_dir='/tmp/snap/amazon0601/', address='https://snap.stanford.edu/data/amazon0601.txt.gz')},
'web_NotreDame': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_NotreDame', tmp_dir='/tmp/snap/web_NotreDame/', address='https://snap.stanford.edu/data/web-NotreDame.txt.gz')},
'amazon0505': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0505', tmp_dir='/tmp/snap/amazon0505/', address='https://snap.stanford.edu/data/amazon0505.txt.gz')},
'ca_GrQc': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_GrQc', tmp_dir='/tmp/snap/ca_GrQc/', address='https://snap.stanford.edu/data/ca-GrQc.txt.gz')},
'amazon0312': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0312', tmp_dir='/tmp/snap/amazon0312/', address='https://snap.stanford.edu/data/amazon0312.txt.gz')},
'amazon0302': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='amazon0302', tmp_dir='/tmp/snap/amazon0302/', address='https://snap.stanford.edu/data/amazon0302.txt.gz')},
'ca_HepPh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_HepPh', tmp_dir='/tmp/snap/ca_HepPh/', address='https://snap.stanford.edu/data/ca-HepPh.txt.gz')},
'ca_AstroPh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_AstroPh', tmp_dir='/tmp/snap/ca_AstroPh/', address='https://snap.stanford.edu/data/ca-AstroPh.txt.gz')},
'web_Stanford': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_Stanford', tmp_dir='/tmp/snap/web_Stanford/', address='https://snap.stanford.edu/data/web-Stanford.txt.gz')},
'ca_HepTh': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='ca_HepTh', tmp_dir='/tmp/snap/ca_HepTh/', address='https://snap.stanford.edu/data/ca-HepTh.txt.gz')},
'web_Google': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_Google', tmp_dir='/tmp/snap/web_Google/', address='https://snap.stanford.edu/data/web-Google.txt.gz')},
'web_BerkStan': {'dep_list': [],
'command': 'curl {address} -O {tmp_dir}/{file_name}'.format(file_name='web_BerkStan', tmp_dir='/tmp/snap/web_BerkStan/', address='https://snap.stanford.edu/data/web-BerkStan.txt.gz')}}
for (t, params) in params_download_data_from_remote_web_location.items():
    tasks_download_data_from_remote_web_location[t] = ShellTask(command=params_download_data_from_remote_web_location['command'])
    flow.add_node(tasks_download_data_from_remote_web_location[t])
    for dep in params_download_data_from_remote_web_location['dep_list']:
        flow.add_edge(tasks_download_data_from_remote_web_location[t], dep)


download_data_from_remote_gcs_location__sentinel_location = download_blob_to_file('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
flow.add_node(download_data_from_remote_gcs_location__sentinel_location)



params_decompress = {'ca_CondMat': {'dep_list': [tasks_download_data_from_remote_web_location['ca_CondMat']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_CondMat/', file_name='ca_CondMat')},
'ca_GrQc': {'dep_list': [tasks_download_data_from_remote_web_location['ca_GrQc']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_GrQc/', file_name='ca_GrQc')},
'web_BerkStan': {'dep_list': [tasks_download_data_from_remote_web_location['web_BerkStan']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_BerkStan/', file_name='web_BerkStan')},
'amazon0601': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0601']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0601/', file_name='amazon0601')},
'amazon0302': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0302']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0302/', file_name='amazon0302')},
'web_Google': {'dep_list': [tasks_download_data_from_remote_web_location['web_Google']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_Google/', file_name='web_Google')},
'ca_AstroPh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_AstroPh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_AstroPh/', file_name='ca_AstroPh')},
'ca_HepPh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_HepPh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_HepPh/', file_name='ca_HepPh')},
'ca_HepTh': {'dep_list': [tasks_download_data_from_remote_web_location['ca_HepTh']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/ca_HepTh/', file_name='ca_HepTh')},
'web_NotreDame': {'dep_list': [tasks_download_data_from_remote_web_location['web_NotreDame']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_NotreDame/', file_name='web_NotreDame')},
'sentinel': {'dep_list': [download_data_from_remote_gcs_location__sentinel_location],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table')},
'amazon0312': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0312']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0312/', file_name='amazon0312')},
'amazon0505': {'dep_list': [tasks_download_data_from_remote_web_location['amazon0505']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/amazon0505/', file_name='amazon0505')},
'web_Stanford': {'dep_list': [tasks_download_data_from_remote_web_location['web_Stanford']],
'command': 'gunzip {tmp_dir}/{file_name}'.format(tmp_dir='/tmp/snap/web_Stanford/', file_name='web_Stanford')}}
for (t, params) in params_decompress.items():
    tasks_decompress[t] = ShellTask(command=params_decompress['command'])
    flow.add_node(tasks_decompress[t])
    for dep in params_decompress['dep_list']:
        flow.add_edge(tasks_decompress[t], dep)



hive_schemas_created_command = """
presto -e 'CREATE TABLE IF NOT EXISTS {table_name} (
{schema} ) WITH (format='{data_format}');
'"""
hive_schemas_created_schema = """
    from_id BIGINT,
    to_id BIGINT
"""
params_hive_schemas_created = {'sentinel': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='sentinel-2-metadata-table', schema="""
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
'web_Google': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='web_Google', schema=hive_schemas_created_schema)},
'ca_HepTh': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='ca_HepTh', schema=hive_schemas_created_schema)},
'amazon0302': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='amazon0302', schema=hive_schemas_created_schema)},
'ca_AstroPh': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='ca_AstroPh', schema=hive_schemas_created_schema)},
'ca_CondMat': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='ca_CondMat', schema=hive_schemas_created_schema)},
'amazon0312': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='amazon0312', schema=hive_schemas_created_schema)},
'web_BerkStan': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='web_BerkStan', schema=hive_schemas_created_schema)},
'web_Stanford': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='web_Stanford', schema=hive_schemas_created_schema)},
'ca_HepPh': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='ca_HepPh', schema=hive_schemas_created_schema)},
'amazon0601': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='amazon0601', schema=hive_schemas_created_schema)},
'ca_GrQc': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='ca_GrQc', schema=hive_schemas_created_schema)},
'web_NotreDame': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='web_NotreDame', schema=hive_schemas_created_schema)},
'amazon0505': {'dep_list': [],
'command': hive_schemas_created_command.format(data_format='ORC', table_name='amazon0505', schema=hive_schemas_created_schema)}}
for (t, params) in params_hive_schemas_created.items():
    tasks_hive_schemas_created[t] = ShellTask(command=params_hive_schemas_created['command'])
    flow.add_node(tasks_hive_schemas_created[t])
    for dep in params_hive_schemas_created['dep_list']:
        flow.add_edge(tasks_hive_schemas_created[t], dep)


remove_file_header_command = """
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
"""
params_remove_file_header = {'web_Google': {'dep_list': [tasks_decompress['web_Google']],
'command': remove_file_header_command.format(file_name='web_Google', tmp_dir='/tmp/snap/web_Google/', n='2')},
'amazon0302': {'dep_list': [tasks_decompress['amazon0302']],
'command': remove_file_header_command.format(file_name='amazon0302', tmp_dir='/tmp/snap/amazon0302/', n='2')},
'web_Stanford': {'dep_list': [tasks_decompress['web_Stanford']],
'command': remove_file_header_command.format(file_name='web_Stanford', tmp_dir='/tmp/snap/web_Stanford/', n='2')},
'ca_HepTh': {'dep_list': [tasks_decompress['ca_HepTh']],
'command': remove_file_header_command.format(file_name='ca_HepTh', tmp_dir='/tmp/snap/ca_HepTh/', n='2')},
'amazon0312': {'dep_list': [tasks_decompress['amazon0312']],
'command': remove_file_header_command.format(file_name='amazon0312', tmp_dir='/tmp/snap/amazon0312/', n='2')},
'ca_AstroPh': {'dep_list': [tasks_decompress['ca_AstroPh']],
'command': remove_file_header_command.format(file_name='ca_AstroPh', tmp_dir='/tmp/snap/ca_AstroPh/', n='2')},
'sentinel': {'dep_list': [tasks_decompress['sentinel']],
'command': remove_file_header_command.format(file_name='sentinel-2-metadata-table', tmp_dir='/tmp/sentinel2', n='2')},
'web_BerkStan': {'dep_list': [tasks_decompress['web_BerkStan']],
'command': remove_file_header_command.format(file_name='web_BerkStan', tmp_dir='/tmp/snap/web_BerkStan/', n='2')},
'web_NotreDame': {'dep_list': [tasks_decompress['web_NotreDame']],
'command': remove_file_header_command.format(file_name='web_NotreDame', tmp_dir='/tmp/snap/web_NotreDame/', n='2')},
'ca_GrQc': {'dep_list': [tasks_decompress['ca_GrQc']],
'command': remove_file_header_command.format(file_name='ca_GrQc', tmp_dir='/tmp/snap/ca_GrQc/', n='2')},
'amazon0601': {'dep_list': [tasks_decompress['amazon0601']],
'command': remove_file_header_command.format(file_name='amazon0601', tmp_dir='/tmp/snap/amazon0601/', n='2')},
'amazon0505': {'dep_list': [tasks_decompress['amazon0505']],
'command': remove_file_header_command.format(file_name='amazon0505', tmp_dir='/tmp/snap/amazon0505/', n='2')},
'ca_HepPh': {'dep_list': [tasks_decompress['ca_HepPh']],
'command': remove_file_header_command.format(file_name='ca_HepPh', tmp_dir='/tmp/snap/ca_HepPh/', n='2')},
'ca_CondMat': {'dep_list': [tasks_decompress['ca_CondMat']],
'command': remove_file_header_command.format(file_name='ca_CondMat', tmp_dir='/tmp/snap/ca_CondMat/', n='2')}}
for (t, params) in params_remove_file_header.items():
    tasks_remove_file_header[t] = ShellTask(command=params_remove_file_header['command'])
    flow.add_node(tasks_remove_file_header[t])
    for dep in params_remove_file_header['dep_list']:
        flow.add_edge(tasks_remove_file_header[t], dep)


params_file_ready_for_upload = {'ca_GrQc': {'dep_list': [tasks_remove_file_header['ca_GrQc']]},
'amazon0302': {'dep_list': [tasks_remove_file_header['amazon0302']]},
'amazon0505': {'dep_list': [tasks_remove_file_header['amazon0505']]},
'ca_HepPh': {'dep_list': [tasks_remove_file_header['ca_HepPh']]},
'web_BerkStan': {'dep_list': [tasks_remove_file_header['web_BerkStan']]},
'web_Stanford': {'dep_list': [tasks_remove_file_header['web_Stanford']]},
'amazon0601': {'dep_list': [tasks_remove_file_header['amazon0601']]},
'web_Google': {'dep_list': [tasks_remove_file_header['web_Google']]},
'ca_HepTh': {'dep_list': [tasks_remove_file_header['ca_HepTh']]},
'web_NotreDame': {'dep_list': [tasks_remove_file_header['web_NotreDame']]},
'ca_CondMat': {'dep_list': [tasks_remove_file_header['ca_CondMat']]},
'amazon0312': {'dep_list': [tasks_remove_file_header['amazon0312']]},
'ca_AstroPh': {'dep_list': [tasks_remove_file_header['ca_AstroPh']]}}
for (t, params) in params_file_ready_for_upload.items():
    tasks_file_ready_for_upload[t] = ConstantTask('FileReadyForUpload')
    flow.add_node(tasks_file_ready_for_upload[t])
    for dep in params_file_ready_for_upload['dep_list']:
        flow.add_edge(tasks_file_ready_for_upload[t], dep)


convert_csv_to_orc_command = """
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
"""
params_convert_csv_to_orc = {'sentinel': {'dep_list': [tasks_remove_file_header['sentinel']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/sentinel2', table_name='sentinel-2-metadata-table', schema=('granule_id:STRING,product_id:STRING'
',datatake_identifier:STRING,mgrs_tile:STRING'
',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
',geometric_quality_flag:STRING,generation_time:BIGINT'
',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT'
',east_lon:FLOAT,base_url:STRING'))},
'web_NotreDame': {'dep_list': [tasks_remove_file_header['web_NotreDame']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_NotreDame/', table_name='web_NotreDame', schema='from_id:BIGINT,to_id:BIGINT')},
'web_BerkStan': {'dep_list': [tasks_remove_file_header['web_BerkStan']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_BerkStan/', table_name='web_BerkStan', schema='from_id:BIGINT,to_id:BIGINT')},
'amazon0302': {'dep_list': [tasks_remove_file_header['amazon0302']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0302/', table_name='amazon0302', schema='from_id:BIGINT,to_id:BIGINT')},
'amazon0312': {'dep_list': [tasks_remove_file_header['amazon0312']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0312/', table_name='amazon0312', schema='from_id:BIGINT,to_id:BIGINT')},
'ca_GrQc': {'dep_list': [tasks_remove_file_header['ca_GrQc']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_GrQc/', table_name='ca_GrQc', schema='from_id:BIGINT,to_id:BIGINT')},
'web_Stanford': {'dep_list': [tasks_remove_file_header['web_Stanford']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_Stanford/', table_name='web_Stanford', schema='from_id:BIGINT,to_id:BIGINT')},
'amazon0601': {'dep_list': [tasks_remove_file_header['amazon0601']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0601/', table_name='amazon0601', schema='from_id:BIGINT,to_id:BIGINT')},
'ca_HepPh': {'dep_list': [tasks_remove_file_header['ca_HepPh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_HepPh/', table_name='ca_HepPh', schema='from_id:BIGINT,to_id:BIGINT')},
'amazon0505': {'dep_list': [tasks_remove_file_header['amazon0505']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/amazon0505/', table_name='amazon0505', schema='from_id:BIGINT,to_id:BIGINT')},
'ca_AstroPh': {'dep_list': [tasks_remove_file_header['ca_AstroPh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_AstroPh/', table_name='ca_AstroPh', schema='from_id:BIGINT,to_id:BIGINT')},
'ca_HepTh': {'dep_list': [tasks_remove_file_header['ca_HepTh']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_HepTh/', table_name='ca_HepTh', schema='from_id:BIGINT,to_id:BIGINT')},
'ca_CondMat': {'dep_list': [tasks_remove_file_header['ca_CondMat']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/ca_CondMat/', table_name='ca_CondMat', schema='from_id:BIGINT,to_id:BIGINT')},
'web_Google': {'dep_list': [tasks_remove_file_header['web_Google']],
'command': convert_csv_to_orc_command.format(tmp_dir='/tmp/snap/web_Google/', table_name='web_Google', schema='from_id:BIGINT,to_id:BIGINT')}}
for (t, params) in params_convert_csv_to_orc.items():
    tasks_convert_csv_to_orc[t] = ShellTask(command=params_convert_csv_to_orc['command'])
    flow.add_node(tasks_convert_csv_to_orc[t])
    for dep in params_convert_csv_to_orc['dep_list']:
        flow.add_edge(tasks_convert_csv_to_orc[t], dep)


params_upload_data_to_local = {'amazon0302': {'dep_list': [tasks_convert_csv_to_orc['amazon0302']]},
'web_NotreDame': {'dep_list': [tasks_convert_csv_to_orc['web_NotreDame']]},
'ca_HepPh': {'dep_list': [tasks_convert_csv_to_orc['ca_HepPh']]},
'amazon0505': {'dep_list': [tasks_convert_csv_to_orc['amazon0505']]},
'ca_CondMat': {'dep_list': [tasks_convert_csv_to_orc['ca_CondMat']]},
'web_BerkStan': {'dep_list': [tasks_convert_csv_to_orc['web_BerkStan']]},
'amazon0312': {'dep_list': [tasks_convert_csv_to_orc['amazon0312']]},
'web_Google': {'dep_list': [tasks_convert_csv_to_orc['web_Google']]},
'amazon0601': {'dep_list': [tasks_convert_csv_to_orc['amazon0601']]},
'web_Stanford': {'dep_list': [tasks_convert_csv_to_orc['web_Stanford']]},
'ca_HepTh': {'dep_list': [tasks_convert_csv_to_orc['ca_HepTh']]},
'ca_AstroPh': {'dep_list': [tasks_convert_csv_to_orc['ca_AstroPh']]},
'ca_GrQc': {'dep_list': [tasks_convert_csv_to_orc['ca_GrQc']]}}
for (t, params) in params_upload_data_to_local.items():
    tasks_upload_data_to_local[t] = ConstantTask('UploadDataToLocal')
    flow.add_node(tasks_upload_data_to_local[t])
    for dep in params_upload_data_to_local['dep_list']:
        flow.add_edge(tasks_upload_data_to_local[t], dep)


params_replicated_schema = {'amazon0312': {'dep_list': [tasks_hive_schemas_created['amazon0312']]},
'ca_HepTh': {'dep_list': [tasks_hive_schemas_created['ca_HepTh']]},
'web_NotreDame': {'dep_list': [tasks_hive_schemas_created['web_NotreDame']]},
'amazon0505': {'dep_list': [tasks_hive_schemas_created['amazon0505']]},
'sentinel': {'dep_list': [tasks_hive_schemas_created['sentinel']]},
'web_BerkStan': {'dep_list': [tasks_hive_schemas_created['web_BerkStan']]},
'ca_HepPh': {'dep_list': [tasks_hive_schemas_created['ca_HepPh']]},
'ca_CondMat': {'dep_list': [tasks_hive_schemas_created['ca_CondMat']]},
'amazon0302': {'dep_list': [tasks_hive_schemas_created['amazon0302']]},
'ca_GrQc': {'dep_list': [tasks_hive_schemas_created['ca_GrQc']]},
'amazon0601': {'dep_list': [tasks_hive_schemas_created['amazon0601']]},
'ca_AstroPh': {'dep_list': [tasks_hive_schemas_created['ca_AstroPh']]},
'web_Stanford': {'dep_list': [tasks_hive_schemas_created['web_Stanford']]},
'web_Google': {'dep_list': [tasks_hive_schemas_created['web_Google']]}}
for (t, params) in params_replicated_schema.items():
    tasks_replicated_schema[t] = ConstantTask('ReplicatedSchema')
    flow.add_node(tasks_replicated_schema[t])
    for dep in params_replicated_schema['dep_list']:
        flow.add_edge(tasks_replicated_schema[t], dep)


replicated_data__sentinel = ConstantTask('ReplicatedData')
flow.add_node(replicated_data__sentinel)
for dep in [tasks_remove_file_header['sentinel'], tasks_convert_csv_to_orc['sentinel']]:
    flow.add_edge(replicated_data__sentinel, dep)


params_replicated_assets = {'amazon0312': {'dep_list': [tasks_file_ready_for_upload['amazon0312'], tasks_upload_data_to_local['amazon0312'], tasks_replicated_schema['amazon0312']]},
'ca_GrQc': {'dep_list': [tasks_file_ready_for_upload['ca_GrQc'], tasks_upload_data_to_local['ca_GrQc'], tasks_replicated_schema['ca_GrQc']]},
'web_Stanford': {'dep_list': [tasks_file_ready_for_upload['web_Stanford'], tasks_upload_data_to_local['web_Stanford'], tasks_replicated_schema['web_Stanford']]},
'web_BerkStan': {'dep_list': [tasks_file_ready_for_upload['web_BerkStan'], tasks_upload_data_to_local['web_BerkStan'], tasks_replicated_schema['web_BerkStan']]},
'ca_HepPh': {'dep_list': [tasks_file_ready_for_upload['ca_HepPh'], tasks_upload_data_to_local['ca_HepPh'], tasks_replicated_schema['ca_HepPh']]},
'web_Google': {'dep_list': [tasks_file_ready_for_upload['web_Google'], tasks_upload_data_to_local['web_Google'], tasks_replicated_schema['web_Google']]},
'ca_CondMat': {'dep_list': [tasks_file_ready_for_upload['ca_CondMat'], tasks_upload_data_to_local['ca_CondMat'], tasks_replicated_schema['ca_CondMat']]},
'amazon0302': {'dep_list': [tasks_file_ready_for_upload['amazon0302'], tasks_upload_data_to_local['amazon0302'], tasks_replicated_schema['amazon0302']]},
'amazon0505': {'dep_list': [tasks_file_ready_for_upload['amazon0505'], tasks_upload_data_to_local['amazon0505'], tasks_replicated_schema['amazon0505']]},
'ca_HepTh': {'dep_list': [tasks_file_ready_for_upload['ca_HepTh'], tasks_upload_data_to_local['ca_HepTh'], tasks_replicated_schema['ca_HepTh']]},
'web_NotreDame': {'dep_list': [tasks_file_ready_for_upload['web_NotreDame'], tasks_upload_data_to_local['web_NotreDame'], tasks_replicated_schema['web_NotreDame']]},
'amazon0601': {'dep_list': [tasks_file_ready_for_upload['amazon0601'], tasks_upload_data_to_local['amazon0601'], tasks_replicated_schema['amazon0601']]},
'ca_AstroPh': {'dep_list': [tasks_file_ready_for_upload['ca_AstroPh'], tasks_upload_data_to_local['ca_AstroPh'], tasks_replicated_schema['ca_AstroPh']]}}
for (t, params) in params_replicated_assets.items():
    tasks_replicated_assets[t] = ConstantTask('ReplicatedAssets')
    flow.add_node(tasks_replicated_assets[t])
    for dep in params_replicated_assets['dep_list']:
        flow.add_edge(tasks_replicated_assets[t], dep)


replicated_data_sets__sentinel = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__sentinel)
for dep in [tasks_replicated_schema['sentinel'], replicated_data__sentinel]:
    flow.add_edge(replicated_data_sets__sentinel, dep)

replicated_data_sets__snap = ConstantTask('ReplicatedDataSets')
flow.add_node(replicated_data_sets__snap)
for dep in [tasks_replicated_assets['amazon0505'], tasks_replicated_assets['amazon0302'], tasks_replicated_assets['ca_AstroPh'], tasks_replicated_assets['web_Google'], tasks_replicated_assets['web_Stanford'], tasks_replicated_assets['amazon0601'], tasks_replicated_assets['ca_HepPh'], tasks_replicated_assets['web_NotreDame'], tasks_replicated_assets['web_BerkStan'], tasks_replicated_assets['ca_HepTh'], tasks_replicated_assets['ca_CondMat'], tasks_replicated_assets['amazon0312'], tasks_replicated_assets['ca_GrQc']]:
    flow.add_edge(replicated_data_sets__snap, dep)


is_consistent__basic_data_setup = ConstantTask('IsConsistent')
flow.add_node(is_consistent__basic_data_setup)
for dep in [replicated_data_sets__sentinel, replicated_data_sets__snap]:
    flow.add_edge(is_consistent__basic_data_setup, dep)


params_is_audited_table = {'amazon0302': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'web_Google': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'ca_HepPh': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'amazon0312': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'web_BerkStan': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'amazon0601': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'ca_GrQc': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'ca_HepTh': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'web_NotreDame': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'web_Stanford': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'amazon0505': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'ca_AstroPh': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'sentinel': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]},
'ca_CondMat': {'dep_list': [replicated_data_sets__sentinel, replicated_data_sets__snap]}}
for (t, params) in params_is_audited_table.items():
    tasks_is_audited_table[t] = ConstantTask('IsAuditedTable')
    flow.add_node(tasks_is_audited_table[t])
    for dep in params_is_audited_table['dep_list']:
        flow.add_edge(tasks_is_audited_table[t], dep)


is_audited__basic_data_setup = ConstantTask('IsAudited')
flow.add_node(is_audited__basic_data_setup)
for dep in [tasks_is_audited_table['amazon0505'], tasks_is_audited_table['web_BerkStan'], tasks_is_audited_table['ca_HepPh'], tasks_is_audited_table['amazon0601'], tasks_is_audited_table['ca_CondMat'], tasks_is_audited_table['ca_GrQc'], tasks_is_audited_table['amazon0302'], tasks_is_audited_table['ca_HepTh'], tasks_is_audited_table['amazon0312'], tasks_is_audited_table['sentinel'], tasks_is_audited_table['web_Google'], tasks_is_audited_table['web_NotreDame'], tasks_is_audited_table['web_Stanford'], tasks_is_audited_table['ca_AstroPh']]:
    flow.add_edge(is_audited__basic_data_setup, dep)


