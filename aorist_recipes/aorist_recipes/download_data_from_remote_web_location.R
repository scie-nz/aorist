###+
# @aorist_r(
#     programs,
#     DownloadDataFromRemoteWebLocation,
#     entrypoint="download.data.from.remote.web.location",
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
download.data.from.remote.web.location <- function(
    src_url, tmp_dir, dest_file_name, table_name, dataset_name
) {
    if (!dir.exists(tmp_dir)) {
        dir.create(tmp_dir, recursive=TRUE)
    }
    dest <- paste(tmp_dir, dest_file_name, sep='/')
    download.file(src_url, dest)
}
