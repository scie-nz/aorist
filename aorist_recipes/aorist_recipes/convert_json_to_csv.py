from aorist import aorist, ConvertJSONToCSV

programs = {}

@aorist(
    programs,
    ConvertJSONToCSV,
    entrypoint="convert_json_to_csv",
    args={
        "src_file_name": lambda context: (
            context.get("json_file"),
            context
        ),
        "dest_file_name": lambda context: (
            context.capture(
                "csv_file", context.get("json_file").replace(".json", ".csv"),
            ),
            context
        ),
    },
)
def recipe(src_file_name, dest_file_name):
    import os

    def convert_json_to_csv(src_file_name, dest_file_name):
        # TODO: for now this just moves the JSON file and doesn't actually convert it to CSV
        os.rename(os.path.join(tmp_dir, src_file_name), os.path.join(tmp_dir, dest_file_name))
