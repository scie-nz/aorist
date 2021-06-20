from aorist import aorist, RenameCSV

programs = {}

@aorist(
    programs,
    RenameCSV,
    entrypoint="rename_csv",
    args={
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "src_file_name": lambda lng: lng.{
  let file_name = &static_data_table.name;
  match &remote_storage.encoding.get_compression() {
      None => format!("{}.txt", file_name),
      Some(_) => format!("{}.downloaded", file_name),
  }
}
,
        "dest_file_name": lambda lng: lng.format!("{}.csv", static_data_table.name),
    },
)
def recipe(tmp_dir, src_file_name, dest_file_name):
    import os
    
    def rename_csv(tmp_dir, src_file_name, dest_file_name):
        os.rename(os.path.join(tmp_dir, src_file_name), os.path.join(tmp_dir, dest_file_name))
    
    