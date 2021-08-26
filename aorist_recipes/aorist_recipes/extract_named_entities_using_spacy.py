from aorist import aorist, ExtractNamedEntitiesUsingSpacy

programs = {}

@aorist(
    programs,
    ExtractNamedEntitiesUsingSpacy,
    entrypoint="extract_named_entities",
    args={
        "spacy_model": lambda spacy_named_entity_schema: spacy_named_entity_schema.spacy_model_name,
        "text_data_file": lambda context: (context.get("text_data_file"), context),
        "ner_file": lambda named_entities, context: (
            context.capture(
                "file_to_replicate",
                named_entities.setup.local_storage_setup.tmp_dir + "/named_entities.txt",
            ),
            context
        ),
        "_is_json": lambda context: (context.capture_bool("is_json", True), context),
        "_delimiter": lambda context: (context.capture("delimiter", ""), context),
    },
)
def recipe(
    text_data_file, dim, ner_file,
):
    import spacy

    def extract_named_entities(text_data_file, spacy_model, ner_file):
        nlp = spacy.load(spacy_model)
        with open(text_data_file) as infile, open(ner_file, 'w') as outfile:
            for (i, line) in enumerate(infile.readlines()):
                doc = nlp(line)
                for (j, ent) in enumerate(doc.ents):
                    outfile.write(json.dumps(
                        {
                            "line_id": i,
                            "entity_id": j,
                            "entity_text": ent.text,
                            "text": line,
                            "start": ent.start_char,
                            "end": ent.end_char,
                            "label": ent.label_,
                        }
                    ) + chr(10))
            
