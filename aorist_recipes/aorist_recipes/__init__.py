from . import download_data_from_remote_gcs_location
from . import download_data_from_remote_web_location
from . import download_data_from_remote_pushshift_api_location_to_newline_delimited_json
from . import extract_named_entities_using_spacy
from . import convert_json_to_csv
from . import text_corpus_data_from_hive
from . import text_corpus_data_from_sqlite
from . import train_fasttext_model
from . import upload_data_to_minio
from . import upload_fasttext_to_minio
from . import upload_data_to_sqlite
import pathlib
from aorist import register_recipes


def bash_module(filename):
    text = open(filename).read()
    assert(text.index('###+') == 0)
    b = text[1:].index('###+')
    decorator = text[5:b].replace("# ", "")
    program = decorator[1:]
    entrypoint = text[b + 5:].strip()

    tree = ast.parse(program)
    assert(len(tree.body[0].value.args) == 2)
    constraint = tree.body[0].value.args[1]
    tree.body[0].value.args += [ast.Constant(entrypoint)]
    tree.body.insert(0, ast.Import(
        [ast.alias(name="builtins", asname=None)],
    ))
    tree.body.insert(0, ast.ImportFrom("aorist", [
        ast.alias(name="aorist_bash", asname=None),
        ast.alias(name=constraint.id, asname=None),
    ], 0))
    tree.body.insert(2, ast.Assign(
        [ast.Attribute(
            ast.Name(id="builtins", ctx=ast.Load()),
            constraint.id,
            ast.Store(),
        )],
        ast.Name(id=constraint.id, ctx=ast.Load()),
    ))
    tree.body.insert(3, ast.Assign(
        [ast.Name(id="programs", ctx=ast.Store())],
        ast.Dict([],[]),
    ))
    code = astor.to_source(tree)
    module_name = filename.replace('.sh', '').split('/')[-1]
    module = imp.new_module(module_name)
    exec(code, module.__dict__)
    return module


path = pathlib.Path(__file__).parent.resolve()

programs = register_recipes(
    py_modules=[
        download_data_from_remote_pushshift_api_location_to_newline_delimited_json,
        download_data_from_remote_gcs_location,
        download_data_from_remote_web_location,
        upload_data_to_minio,
        convert_json_to_csv,
        text_corpus_data_from_hive,
        text_corpus_data_from_sqlite,
        train_fasttext_model,
        upload_fasttext_to_minio,
        upload_data_to_sqlite,
        extract_named_entities_using_spacy,
    ],
    sql_modules=[
        "%s/hive_directories_created.presto.sql" % path,
        "%s/json_table_schemas_created.presto.sql" % path,
        "%s/convert_json_table_to_orc_table.presto.sql" % path,
        "%s/orc_table_schemas_created.presto.sql" % path,
    ],
    bash_modules=[
        "%s/download_data_from_remote_web_location.sh" % path,
    ],
    r_modules=[
        "%s/download_data_from_remote_web_location.R" % path,
    ],
)

