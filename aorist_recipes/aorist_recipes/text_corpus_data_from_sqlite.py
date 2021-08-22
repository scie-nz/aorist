from aorist import aorist, TextCorpusDataFromSQLite
from json import dumps

programs = {}

@aorist(
    programs,
    TextCorpusDataFromSQLite,
    entrypoint="download_text_data_from_sqlite",
    args={
        "text_attribute_name": lambda language_asset: language_asset.schema.language_asset_schema.text_attribute_name,
        "source_tables": lambda language_asset: dumps([
            x.name() for x in language_asset.source_assets
        ]),
        "dedup_text_attribute": lambda language_asset: language_asset.schema.language_asset_schema.should_dedup_text_attribute(),
        "db_filename": lambda language_asset: \
            language_asset.storage_setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "text_data_file": lambda language_asset, context: (
            context.capture(
                "text_data_file",
                language_asset.storage_setup.local_storage_setup.tmp_dir + "/training_data.txt",
            ),
            context
        ),
        "tmp_dir": lambda language_asset: language_asset.storage_setup.local_storage_setup.tmp_dir,
    },
)
def recipe(
    text_attribute_name, source_tables, text_data_file, db_filename, tmp_dir, dedup_text_attribute,
):
    import sqlite3
    import json

    def download_text_data_from_sqlite(
        text_attribute_name, source_tables, text_data_file, db_filename, tmp_dir, dedup_text_attribute,
    ):
     
        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)
        con = sqlite3.connect(db_filename)
        cursor = con.execute((chr(10) + "UNION ALL" + chr(10)).join([
            "SELECT {dedup}{text_attribute_name} FROM {source_table}".format(
                text_attribute_name=text_attribute_name,
                source_table=source_table,
                dedup="DISTINCT " if dedup_text_attribute else "",
            )
            for source_table in json.loads(source_tables)
        ]))
        with open(text_data_file, 'w') as f: 
            for (text) in cursor:
                f.write('%s' % text)
                f.write(chr(10))
            
