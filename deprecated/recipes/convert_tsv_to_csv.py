from aorist import aorist, ConvertTSVToCSV

programs = {}

@aorist(
    programs,
    ConvertTSVToCSV,
    entrypoint="tsv_to_csv",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "table_name": lambda lng: lng.static_data_table.name,
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
def recipe(dataset_name, table_name, tmp_dir, src_file_name, dest_file_name):
    import os.path
    
    def tsv_to_csv(tmp_dir, src_file_name, dest_file_name, dataset_name, table_name):
        source = os.path.join(tmp_dir, dataset_name, table_name, src_file_name)
        dest = os.path.join(tmp_dir, dataset_name, table_name, dest_file_name)
        with open(source, 'rb') as sourcef, open(dest, 'w') as destf:
                while True:
                    chunk = sourcef.read(10240)
                    chunk = chunk.decode('latin-1')
                    if not chunk:
                        destf.flush()
                        destf.close()
                        break
                    destf.write(chunk.replace('\\t', ','))
    
    