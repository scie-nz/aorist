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


tasks['replicated_schema'] = ConstantTask('ReplicatedSchema')
flow.add_node(tasks['replicated_schema'])
params_download_data_from_remote_gcs_location = {
    'download_location': ('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
}
    
tasks['download_location'] = download_blob_to_file('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
flow.add_node(tasks['download_location'])
tasks['download_remote'] = ConstantTask('DownloadDataFromRemote')
flow.add_node(tasks['download_remote'])
flow.add_edge(tasks['download_remote'], tasks['download_location'])
params_decompress = {
    'decompress': ('/tmp/sentinel2', 'sentinel-2-metadata-table')
}
    
tasks['decompress'] = ShellTask(command='unzip_file %s %s' % ('/tmp/sentinel2', 'sentinel-2-metadata-table'))
flow.add_node(tasks['decompress'])
flow.add_edge(tasks['decompress'], tasks['download_location'])
tasks['remove_header'] = ConstantTask('RemoveFileHeader')
flow.add_node(tasks['remove_header'])
flow.add_edge(tasks['remove_header'], tasks['decompress'])
tasks['replicated_data'] = ConstantTask('ReplicatedData')
flow.add_node(tasks['replicated_data'])
for dep in [tasks['download_location'], tasks['download_remote'], tasks['remove_header']]:
    flow.add_edge(tasks['replicated_data'], dep)
tasks['is_consistent'] = ConstantTask('IsConsistent')
flow.add_node(tasks['is_consistent'])
for dep in [tasks['replicated_schema'], tasks['replicated_data']]:
    flow.add_edge(tasks['is_consistent'], dep)
tasks['is_audited'] = ConstantTask('IsAudited')
flow.add_node(tasks['is_audited'])
for dep in [tasks['replicated_schema'], tasks['replicated_data']]:
    flow.add_edge(tasks['is_audited'], dep)
