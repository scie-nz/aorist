from google.cloud import storage
def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_name):
  client = storage.Client.from_service_account_json('/gcloud/social_norms.json')
  bucket = client.bucket(bucket_name)
  blob = bucket.blob(blob_name)
  dest = "%s/%s" % (tmp_dir, file_name)
  blob.download_to_filename(dest)


function unzip_file() {
    gunzip $1/$2
}


params_download_data_from_remote_gcs_location = {
    'download_location': ('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
}
    
tasks['download_location'] = download_blob_to_file(*params_download_data_from_remote_gcs_location['download_location'])
flow.add_node(tasks['download_location'])

tasks['download_remote'] = ConstantTask('download_data_from_remote')
flow.add_node(tasks['download_remote'])
for dep in [tasks['download_location']]:
    flow.add_edge(dep, tasks['download_remote'])

params_decompress = {
    'decompress': ('/tmp/sentinel2', 'sentinel-2-metadata-table')
}
    
flow.add_node(tasks['decompress'])
tasks['decompress'] = ShellTask(
    command="""
    function unzip_file() {
    gunzip $1/$2
}

unzip_file %s
    """ % params_decompress['decompress'].join(' '),
)
flow.add_node(tasks['decompress'])
for dep in [tasks['download_location']]:
    flow.add_edge(dep, tasks['decompress'])

tasks['remove_header'] = ConstantTask('remove_file_header')
flow.add_node(tasks['remove_header'])
for dep in [tasks['decompress']]:
    flow.add_edge(dep, tasks['remove_header'])

tasks['replicated_schema'] = ConstantTask('replicated_schema')
flow.add_node(tasks['replicated_schema'])

tasks['replicated_data'] = ConstantTask('replicated_data')
flow.add_node(tasks['replicated_data'])
for dep in [tasks['download_location'], tasks['download_remote'], tasks['remove_header']]:
    flow.add_edge(dep, tasks['replicated_data'])

tasks['is_consistent'] = ConstantTask('is_consistent')
flow.add_node(tasks['is_consistent'])
for dep in [tasks['replicated_schema'], tasks['replicated_data']]:
    flow.add_edge(dep, tasks['is_consistent'])

tasks['is_audited'] = ConstantTask('is_audited')
flow.add_node(tasks['is_audited'])
for dep in [tasks['replicated_schema'], tasks['replicated_data']]:
    flow.add_edge(dep, tasks['is_audited'])

