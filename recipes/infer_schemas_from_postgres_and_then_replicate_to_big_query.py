from aorist import aorist, InferSchemasFromPostgresAndThenReplicateToBigQuery

programs = {}

@aorist(
    programs,
    InferSchemasFromPostgresAndThenReplicateToBigQuery,
    entrypoint="rebuild_universe",
    args={
        "datasets": lambda lng: lng.serde_json::json!(universe.datasets.iter().map(|x| x.name).collect::<Vec<String>>()),
        "name": lambda lng: lng.universe.name,
    },
)
def recipe(datasets, name):
    def rebuild_universe(name, datasets):
        import json
        datasets = json.loads(datasets)
    
        code = [
            """
            tmp_dir = "/tmp/{name}"
            """.format(name=name),
            """
            local = BigQueryStorage(
                location=BigQueryLocation(),
                layout=StaticTabularLayout(),
            )
            """,
            """
            {name} = Universe(
                name="{name}",
                datasets=[{datasets}],
                endpoints=endpoints,
            )
            """.format(
                name=name,
                datasets=", ".join([
                  '%s.replicate_to_local(local, tmp_dir, CSVEncoding())' % x for x in datasets
                ])
            ),
            """
            {name}.jupyter_extend("UploadDataToBigQuery")
            """.format(name=name),
        ]
        import black
        code = "\\n".join([black.format_str(x.strip(), mode=black.Mode()) for x in code])
    
        from IPython.display import Javascript, display
        import base64
        encoded_code = (base64.b64encode(str.encode(code))).decode()
        display(Javascript("""
            var code = IPython.notebook.insert_cell_at_bottom('code');
            code.set_text(atob( "{0}" ));
            var index = IPython.notebook.get_selected_index();
            IPython.notebook.select(index);
            IPython.notebook.delete_cell();
            """.format(encoded_code)))
    
    