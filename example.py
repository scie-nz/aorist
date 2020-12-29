download_location = download_blob_to_file('gcp-public-data-sentinel2', 'index.csv.gz-backup', '/tmp/sentinel2', 'sentinel-2-metadata-table')
flow.add_node(download_location)
download_remote = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_remote)
flow.add_edge(download_remote, download_location)
decompress = ShellTask(command='gunzip {tmp_dir}/{file_name} ' % ())
flow.add_node(decompress)
flow.add_edge(decompress, download_location)

remove_header = ConstantTask('RemoveFileHeader')
flow.add_node(remove_header)
flow.add_edge(remove_header, decompress)


replicated_data = ConstantTask('ReplicatedData')
flow.add_node(replicated_data)
for dep in [download_location, download_remote, remove_header]:
    flow.add_edge(replicated_data, dep)
replicated_schema = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema)


is_consistent = ConstantTask('IsConsistent')
flow.add_node(is_consistent)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(is_consistent, dep)

is_audited = ConstantTask('IsAudited')
flow.add_node(is_audited)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(is_audited, dep)
