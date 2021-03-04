from scienz import wine

from aorist import (
    MinioLocation,
    StaticHiveTableLayout,
    ORCEncoding,
    SingleFileLayout,
    HiveTableStorage,
    StaticDataTable,
    default_tabular_schema,
    TrainedFloatMeasure,
    ComputedFromLocalData,
    SVMRegressionAlgorithm,
    SupervisedModel,
    ONNXEncoding,
    LocalFileStorage,
)

"""
Defining dataset
"""
# This data is to be replicated locally
local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
wine_dataset = wine.replicate_to_local(
    tmp_dir="/tmp/wine",
    storage=local,
)

wine_table = wine_dataset.get_static_data_table('wine_table')
# We will train a classifier and store it in a local file.
classifier_storage = LocalFileStorage(
    location=MinioLocation(name="wine"),
    layout=SingleFileLayout(),
    encoding=ONNXEncoding(),
)

# We will use these as the features in our classifier.
attributes = wine_dataset.get_attributes_for_asset('wine_table')
features = attributes[2:]

# This is the "recipe" for our classifier.
classifier_template = TrainedFloatMeasure(
    name="predicted_alcohol",
    comment="""
    Predicted alcohol content, based on the following inputs:
    %s
    """ % [x.name for x in features],
    features=features,
    objective=attributes[1],
    source_asset_name="wine_table",
)
# We now augment the dataset with this recipe.
wine_dataset.add_template(classifier_template)
# The classifier is computed from local data
# (note the source_asset_names dictionary)
classifier_setup = ComputedFromLocalData(
    source_asset_names={"training_dataset": "wine_table"},
    target=classifier_storage,
    tmp_dir="/tmp/wine_classifier",
)
# We finally define our regression_model as a concrete
# data asset, following a recipe defined by the template,
# while also connected to concrete storage, as defined
# by classifier_setup
regression_model = SupervisedModel(
    name="wine_alcohol_predictor",
    tag="predictor",
    setup=classifier_setup,
    # TODO: change this to model prediction schema
    schema=classifier_template.get_model_storage_tabular_schema(),
    algorithm=SVMRegressionAlgorithm(),
)
wine_dataset.add_asset(regression_model)


# We now prepare to make predictions based on the classifier
# we just trained. The abstract recipe for this is a predictions
# template (this assumes a trained model and an input data asset
# with a compatible schema for feature requirements).
predictions_template = classifier_template.as_predictions_template(
    name="alcohol_predictions"
)
wine_dataset.add_template(predictions_template)

# We will store our predictions in a Hive table
predictions_storage = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
    tag="wine_predictions_storage",
)
# Our predictions will be computed from local data
predictions_setup = ComputedFromLocalData(
    source_asset_names={
        "model": "wine_alcohol_predictor",
        "source": "wine_table",
    },
    target=predictions_storage,
    tmp_dir="/tmp/wine_predictions",
)
# Our predictions will be available as a static data
# table (which will also include the features).
# Note: the schema has columns in a specific order.
predictions_table = StaticDataTable(
    tag="wine_predictions",
    name="wine_predictions",
    schema=default_tabular_schema(predictions_template),
    setup=predictions_setup,
)
wine_dataset.add_asset(predictions_table)
