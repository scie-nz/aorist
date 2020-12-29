download_location = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(download_location)
download_remote = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_remote)
flow.add_edge(download_location, download_remote)
decompress = ShellTask(command='gunzip {tmp_dir}/{file_name}'.format(
    file_name='sentinel-2-metadata-table', tmp_dir='/tmp/sentinel2'))
flow.add_node(decompress)
flow.add_edge(download_location, decompress)

remove_header = ShellTask(command="""
tail -n +2 {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table'))
flow.add_node(remove_header)
flow.add_edge(decompress, remove_header)


replicated_data = ConstantTask('ReplicatedData')
flow.add_node(replicated_data)
for dep in [download_location, download_remote, remove_header]:
    flow.add_edge(dep, replicated_data)
replicated_schema = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema)


is_consistent = ConstantTask('IsConsistent')
flow.add_node(is_consistent)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(dep, is_consistent)

is_audited = ConstantTask('IsAudited')
flow.add_node(is_audited)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(dep, is_audited)
