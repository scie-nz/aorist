from aorist import aorist, ConvertJSONToCSV

programs = {}

@aorist(
    programs,
    ConvertJSONToCSV,
    entrypoint="convert_json_to_csv",
    args={
        "tmp_dir": lambda replication_storage_setup: replication_storage_setup.tmp_dir,
        "dest_file_name": lambda static_data_table: "%s.csv" % static_data_table.name,
        "src_file_name": lambda static_data_table, remote_storage: static_data_table.name + (
            ".downloaded" if remote_storage.encoding.get_compression() != None else ".txt"
        )
    },
)
def recipe(tmp_dir, src_file_name, dest_file_name):
    import os

    def convert_json_to_csv(tmp_dir, src_file_name, dest_file_name):
        # TODO: for now this just moves the JSON file and doesn't actually convert it to CSV
        os.rename(os.path.join(tmp_dir, src_file_name), os.path.join(tmp_dir, dest_file_name))
