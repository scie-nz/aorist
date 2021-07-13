###+
# @aorist_bash(
#     programs,
#     DownloadDataFromRemoteWebLocation,
#     args={
#         "dataset_name": lambda data_set: data_set.name,
#         "table_name": lambda static_data_table: static_data_table.name,
#         "src_url": lambda web_location: web_location.address,
#         "tmp_dir": lambda replication_storage_setup, context: (
#             context.capture(
#                 "downloaded_tmp_dir",
#                 replication_storage_setup.tmp_dir,
#             ),
#             context,
#         ),
#         "dest_file_name": lambda static_data_table: (
#             "{file_name}.{extension}"
#         ).format(
#             file_name=static_data_table.name,
#             extension=static_data_table.setup.replication_storage_setup.download_extension,
#         )
#     },
# )
###+
mkdir -p {tmp_dir}/{dataset_name}/{table_name} && \
  curl {address} -o {tmp_dir}/{dataset_name}/{table_name}/{file_name}
