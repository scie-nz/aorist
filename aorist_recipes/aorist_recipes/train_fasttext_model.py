from aorist import aorist, TrainFasttextModel
from json import dumps

programs = {}

@aorist(
    programs,
    TrainFasttextModel,
    entrypoint="training_fasttext_model",
    args={
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir,
        "dim": lambda fasttext_embedding_schema: str(fasttext_embedding_schema.dim),
    },
)
def recipe(
    tmp_dir, dim,
):
    import fasttext

    def training_fasttext_model(tmp_dir, dim):
     
        model = fasttext.train_unsupervised('tmp_dir' + 'data.txt', dim=int(dim))
        words = model.get_words()
        
        with open(tmp_dir + 'words.txt', 'w') as f: 
            for (i, word) in words.enumerate():
                f.write(dumps(
                    {
                        "id": i,
                        "word": word,
                        "embedding": model.get_word_vector(word),
                    }
                ))
            
