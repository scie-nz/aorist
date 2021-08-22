from aorist import aorist, TrainFasttextModel

programs = {}

@aorist(
    programs,
    TrainFasttextModel,
    entrypoint="train_fasttext_model",
    args={
        "dim": lambda fasttext_embedding_schema: str(fasttext_embedding_schema.dim),
        "text_data_file": lambda context: (context.get("text_data_file"), context),
        "fasttext_word_embeddings_file": lambda fasttext_embedding, context: (
            context.capture(
                "file_to_replicate",
                fasttext_embedding.setup.local_storage_setup.tmp_dir + "/word_embeddings.txt",
            ),
            context
        )
    },
)
def recipe(
    text_data_file, dim, fasttext_word_embeddings_file,
):
    from fasttext import train_unsupervised
    import json

    def train_fasttext_model(text_data_file, dim, fasttext_word_embeddings_file):
     
        model = train_unsupervised(text_data_file, dim=int(dim))
        words = model.get_words()
        
        with open(fasttext_word_embeddings_file, 'w') as f: 
            for (i, word) in enumerate(words):
                f.write(json.dumps(
                    {
                        "id": i,
                        "word": word,
                        "embedding": model.get_word_vector(word).tolist(),
                    }
                ) + chr(10))
            
