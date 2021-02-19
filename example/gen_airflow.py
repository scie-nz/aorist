from aorist import (
    dag,
    Universe,
    ComplianceConfig,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS
from sentinel import sentinel_dataset
from snap import snap_dataset
from geonames import geonames_dataset, geonames_table
from beers import beers_dataset, beers_table

universe = Universe(
    name="my_cluster",
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    datasets=[
        snap_dataset,
        geonames_dataset,
        beers_dataset,
    ],
    endpoints=DEFAULT_ENDPOINTS,
    compliance=ComplianceConfig(
        description="""
        Testing workflow for data replication of SNAP data to
        local cluster. The SNAP dataset collection is provided
        as open data by Stanford University. The collection contains
        various social and technological network graphs, with
        reasonable and systematic efforts having been made to ensure
        the removal of all Personally Identifiable Information.
        """,
        data_about_human_subjects=True,
        contains_personally_identifiable_information=False,
    ),
    models=[
        SingleObjectiveRegressor(
            name="predict_beer_abv",
            source_data=[beers_table],
            algorithm=RandomForestRegressionAlgorithm(),
            target=LogTransform(
                ContinuousObjective(beers_table.schema.get('abv')),
                base=2
            ),
            validation_config=TrainTestRegressionValidationConfig(test_ratio=0.05),
            feature_groups=[
                CountFeatureGroup(
                    name="top_words_in_name",
                    data=TextToTokenCounts(
                        data=KeepMostFrequent(
                            data=[
                                Tokenize(
                                    Lowercase(
                                        TextFeature(beers_table.schema.get('name')),
                                    )
                                )
                            ],
                            min_frequency=100,
                        ),
                    ),
                ),
                CountFeatureGroup(
                    name="top_words_in_style",
                    data=TextToTokenCounts(
                        data=KeepMostFrequent(
                            data=[
                                Tokenize(
                                    Lowercase(
                                        TextFeature(beers_table.schema.get('style')),
                                    )
                                )
                            ],
                            min_frequency=100,
                        ),
                    ),
                ),
                EmbeddingFeatureGroup(
                    name="name_embeddings",
                    data=FasttextEmbeddings(
                        data=[Lowercase(
                            TextFeature(beers_table.schema.get('style')),
                        )]
                    )
                ),
            ],
        ),
    ],
)
out = dag(universe, [
    "DataDownloadedAndConverted"
], "jupyter")
print(out.replace("\\\\","\\"))
