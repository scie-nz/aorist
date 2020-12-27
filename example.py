function unzip_file() {
    gunzip $1/$2
}


from google.cloud import storage
def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_name):
  client = storage.Client.from_service_account_json('/gcloud/social_norms.json')
  bucket = client.bucket(bucket_name)
  blob = bucket.blob(blob_name)
  dest = "%s/%s" % (tmp_dir, file_name)
  blob.download_to_filename(dest)


for k in [
    'replicated_schema__asset_1__data_set_1',
    'replicated_schema__asset_1__data_set_2'
]:
    tasks[k] = ConstantTask('replicated_schema')
    flow.add_node(tasks[k])

params_download_data_from_remote_gcs_location = {
    'download_data_from_remote_gcs_location': ('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table'),
    'download_data_from_remote_gcs_location__asset_1__data_set_2': ('gcp-public-data-sentinel2', 'index.csv.gz-backup1', '/tmp/sentinel2-1', 'sentinel-2-metadata-table')
}
    
for k, v in params_download_data_from_remote_gcs_location.items():
    tasks[k] = download_blob_to_file(*v)
    flow.add_node(tasks[k])

dependencies_download_data_from_remote = { 
    'download_data_from_remote__asset_1__data_set_2': [
        'download_data_from_remote_gcs_location__asset_1__data_set_2'
    ],
    'download_data_from_remote__asset_1__data_set_1': [
        'download_data_from_remote_gcs_location'
    ] 
}

for k in [
    'download_data_from_remote__asset_1__data_set_2',
    'download_data_from_remote__asset_1__data_set_1'
]:
    tasks[k] = ConstantTask('download_data_from_remote')
    flow.add_node(tasks[k])
    for dep in dependencies_download_data_from_remote[k]:
        flow.add_edge(tasks[dep], tasks[k])

params_decompress = {
    'decompress__asset_1__data_set_2': ('/tmp/sentinel2-1', 'sentinel-2-metadata-table'),
    'decompress__asset_1__data_set_1': ('/tmp/sentinel2', 'sentinel-2-metadata-table')
}
    
dependencies_decompress = { 
    'decompress__asset_1__data_set_2': [
        'download_data_from_remote_gcs_location__asset_1__data_set_2'
    ],
    'decompress__asset_1__data_set_1': [
        'download_data_from_remote_gcs_location'
    ] 
}

for k, v in params_decompress.items():
    tasks[k] = ShellTask(
        command="""
        function unzip_file() {
            gunzip $1/$2
        }
        
        unzip_file %s
        """ % v.join(' '),
    )
    flow.add_node(tasks[k])
    for dep in dependencies_decompress[k]:
        flow.add_edge(tasks[dep], tasks[k])

dependencies_remove_file_header = { 
    'remove_file_header__asset_1__data_set_1': [
        'decompress__asset_1__data_set_1'
    ],
    'remove_file_header__asset_1__data_set_2': [
        'decompress__asset_1__data_set_2'
    ] 
}

for k in [
    'remove_file_header__asset_1__data_set_1',
    'remove_file_header__asset_1__data_set_2'
]:
    tasks[k] = ConstantTask('remove_file_header')
    flow.add_node(tasks[k])
    for dep in dependencies_remove_file_header[k]:
        flow.add_edge(tasks[dep], tasks[k])

dependencies_replicated_data = { 
    'replicated_data__asset_1__data_set_1': [
        'download_data_from_remote_gcs_location',
        'download_data_from_remote__asset_1__data_set_1',
        'remove_file_header__asset_1__data_set_1'
    ],
    'replicated_data__asset_1__data_set_2': [
        'download_data_from_remote_gcs_location__asset_1__data_set_2',
        'download_data_from_remote__asset_1__data_set_2',
        'remove_file_header__asset_1__data_set_2'
    ] 
}

for k in [
    'replicated_data__asset_1__data_set_1',
    'replicated_data__asset_1__data_set_2'
]:
    tasks[k] = ConstantTask('replicated_data')
    flow.add_node(tasks[k])
    for dep in dependencies_replicated_data[k]:
        flow.add_edge(tasks[dep], tasks[k])

dependencies_replicated_data_sets = { 
    'replicated_data_sets__data_set_2': [
        'replicated_schema__asset_1__data_set_2',
        'replicated_data__asset_1__data_set_2'
    ],
    'replicated_data_sets__data_set_1': [
        'replicated_schema__asset_1__data_set_1',
        'replicated_data__asset_1__data_set_1'
    ] 
}

for k in [
    'replicated_data_sets__data_set_2',
    'replicated_data_sets__data_set_1'
]:
    tasks[k] = ConstantTask('replicated_data_sets')
    flow.add_node(tasks[k])
    for dep in dependencies_replicated_data_sets[k]:
        flow.add_edge(tasks[dep], tasks[k])

tasks['is_consistent'] = ConstantTask('is_consistent')
flow.add_node(tasks['is_consistent'])
for dep in [tasks['replicated_data_sets__data_set_2'], tasks['replicated_data_sets__data_set_1']]:
    flow.add_edge(dep, tasks['is_consistent'])

dependencies_is_audited_table = { 
    'is_audited_table__asset_1__data_set_1': [
        'replicated_data_sets__data_set_2',
        'replicated_data_sets__data_set_1'
    ],
    'is_audited_table__asset_1__data_set_2': [
        'replicated_data_sets__data_set_2',
        'replicated_data_sets__data_set_1'
    ] 
}

for k in [
    'is_audited_table__asset_1__data_set_1',
    'is_audited_table__asset_1__data_set_2'
]:
    tasks[k] = ConstantTask('is_audited_table')
    flow.add_node(tasks[k])
    for dep in dependencies_is_audited_table[k]:
        flow.add_edge(tasks[dep], tasks[k])

tasks['is_audited'] = ConstantTask('is_audited')
flow.add_node(tasks['is_audited'])
for dep in [tasks['is_audited_table__asset_1__data_set_1'], tasks['is_audited_table__asset_1__data_set_2']]:
    flow.add_edge(dep, tasks['is_audited'])

