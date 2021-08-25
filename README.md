# Aorist

Aorist is a code-generation tool for MLOps. Its aim is to generate legible
code for common repetitive tasks in data science, such as data replication,
common transformations, as well as machine learning operations.

## Installation instructions

Go to [aorist.io](https://aorist.io) for installation instructions and a tutorial.
You can find the developer guide below.

# Developer Guide

## Package organization

Aorist has a Rust core and a Python interface. The project relies on the following sub-projects:
- `aorist_util` -- a Rust crate containing small utility functions used across the project.
- `aorist_derive` -- Rust crate exporting `derive` macros (and only those macros) used across the project.
- `aorist_primitives` -- Rust crate exporting "primitive" macros (such as `register_constraint`, `define_attribute`, etc.) used to abstract away boiler-plate code inside the Rust code base.
- `aorist_concept` -- a Rust crate dedicated to the `aorist` macro. This macro "decorates" structs and enums to make them "constrainable" in the aorist sense.
- `aorist_ast` -- a Rust crate implementing a cross-language Abstract Syntax Tree (AST), used for generating code in both Python and R. Aorist AST nodes get compiled into native Python or R AST nodes. More languages can be supported here.
- `aorist_attributes` -- this Rust crate exports a taxonomy of data attributes (e.g. `KeyStringIdentifier`, `POSIXTimestamp`), which can be used to impose data quality and compliance constraints across table schemas.
- `aorist_core` -- This is the core Rust crate for the Aorist project. The main object taxonomy is defined here. New structs and enums can be added here.
- `aorist_constraint` -- This Rust crate lists constraints that can be applied to Aorist universes made up of concepts as listed in `aorist_core`. Multiple `aorist_constraint` crates can be compiled against the `aorist_core` crate.
- `aorist` -- This Rust crate exports a Python library via a PyO3 binding. This directory also contains the conda recipe used for creating the `aorist` conda package (which includes the compiled Rust library, as well as a number of Python helpers).
- `aorist_recipes` -- This Python package contains recipes (using Python, TrinoSQL, R, or Bash) that can be used to satisfy constraints as defined in `aorist_constraint`. Multiple `aorist_recipes` packages can be provided at runtime. 
- `scienz` -- This Python package contains a set of pre-defined datasets which can be used out-of-the box with the `aorist` package.

## How to build

Because Aorist is a mixed Rust / Python project, building involves two stages:
- first a set of Rust libraries is built via `cargo`.
- then, a Python library is built bia `conda`.

### Rust library

#### Pre-requisites
You will need to [install Rust](https://www.rust-lang.org/tools/install) in order to compile Aorist.

#### Building
You can build individual Rust libraries directly by running `cargo build` from within the respective directory listed in the
[Package Organization](https://github.com/scie-nz/aorist#package-organization) section.

To build the entire project run `cargo build` from the root directory.

### Conda library

#### Pre-requisites

1. Install Anaconda.

2. Make sure you use conda-forge, rather than the default conda channel.

```
conda config --add channels conda-forge
conda config --set channel_priority strict
```

#### Building

Build the packages by running:

```
cd aorist && conda build . && cd .. && \
cd aorist_recipes && conda build . && cd .. && \
cd scienz && conda build . && cd ..
``` 

### Adding new datasets

You can add new canonical datasets to the `scienz` package. Once accepted for publication metadata associated with these datasets can be distributed painlessly. To do so, please follow the steps described below: 

1. specify your datasets in a new Python file in the `scienz/scienz` directory. (You can look at other files in that directory for examples)
2. make sure to import the datasets in `scienz/__init__.py`.
3. Run `conda build .` from within the `scienz` subdirectory. The build step will also trigger a test, which ensures that your dataset is correctly specified.
4. If `conda build .` succeeds, submit a Pull Request against scienz/aorist.
5. Once the PR is accepted, the `scienz` package will be rebuilt and your dataset will be accessible via Anaconda. 

### How to test

Run the following commands:

```
pip install astor black dill
```
Inside aorist:
```
python build_for_testing.py
```
Inside aorist/scienz:
```
PYTHONPATH=$PYTHONPATH:../aorist_recipes:../scienz:../aorist python run_test.py
``` 
If no error messages appear, your new dataset has been successfully added.

## Overview of an Aorist universe

*(note that the code examples below are provided for illustrative purposes and may have occasional bugs)*

Let's say we are starting a new project which involves analyzing a number of
large graph datasets, such as the ones provided by the
[SNAP](snap.stanford.edu) project.

We will conduct our analysis in a mini data-lake, such as the
[Trino](trino.io) + [MinIO](min.io) solution specified by
[Walden](https://github.com/scie-nz/walden).

We would like to replicate all these graphs into our data lake before we
can start analyzing them. At a very high-level, this is achieved by defining
a "universe", the totality of things we care about in our project. One such
universe is specified below:

```python
from snap import snap_dataset
from aorist import (
    dag,
    Universe,
    ComplianceConfig,
    HiveTableStorage,
    MinioLocation,
    StaticHiveTableLayout,
    ORCEncoding,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS

universe = Universe(
    name="my_cluster",
    datasets=[
        snap_dataset,
    ],
    endpoints=DEFAULT_ENDPOINTS,
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
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
)
```

The universe definition contains a number of things:
- the datasets we are talking about (more about it in a bit),
- the endpoints we have available (e.g. the fact that a MinIO server
  is available for storage, as opposed to HDFS or S3, etc., and where
  that server is available; what endpoint we should use for Presto /
  Trino, etc.)
- who the users and groups are that will access the dataset,
- some compliance annotations.

Note: Currently users, groups, and compliance annotations are supported as a
proof of concept -- these concepts are not essential to an introduction
so we will skip them for now.

## Generating a DAG

To generate a flow that replicates our data all we have to do is run:

```python
DIALECT = "python"
out = dag(
  universe, [
    "AllAssetsComputed",
  ], DIALECT
)
```
This will generate a set of Python tasks, which will do the following, for
each asset (i.e., each graph) in our dataset:

- download it from its remote location,
- decompress it, if necessary
- remove its header,
- convert the file to a CSV, if necessary
- upload the CSV data to MinIO
- create a Hive table backing the MinIO location
- convert the CSV-based Hive table to an ORC-based Hive table
- drop the temporary CSV-based Hive table

This set of tasks is also known as a Directed Acyclic Graph (DAG).
The same DAG can be generated as a Jupyter notebook, e.g. by setting:
```python
DIALECT = "jupyter"
```
Or we can set `DIALECT` to `"airflow"` for an Airflow DAG.


### Aside: what is actually going on?
What Aorist does is quite complex -- the following is an explanation of the
conceptual details, but you can skip this if you'd want something a bit more
concrete:
- first, you describe the universe. This universe is actually a
  highly-structured hierarchy of concepts, each of which can be
  "constrained".
- A constraint is something that "needs to happen". In this example all
  you declare that needs to happen is the constraint
  `AllAssetsComputed`. This constraint is attached to the Universe,
  which is a singleton object.
- Constraints attach to specific kinds of objects -- some attach to the entire
  Universe, others attach to tables, etc.
- Constraints are considered to be satisfied when their dependent constraints
  are satisfied. When we populate each constraint's own dependent constraints
  we follow a set of complex mapping rules that are nonetheless fairly
  intuitive (but difficult to express without a longer discussion, see the end
  of this document for that)
- Programs ("recipes") are attached to this constraint graph by a Driver. The
  Driver decides which languages are prefered (e.g. maybe the Driver likes
  Bash scrips more than Presto, etc.). The driver will complain if it can't
  provide a solution for a particular constraint.
- Once the recipes are attached, various minutiae are extracted from the
  concept hierarchy -- e.g., which endpoints to hit, actual schemas of input
  datasets, etc.
- Once the various minutiae are filled in, we have a graph of Python code
  snippets. If these snippets are repetitive (e.g. 100 instances of the same
  function call but with different arguments) we compress them into for loops
  over parameter dictionarie.
- We then take the compressed snippet-graph and further optimize it, for
  instance by pushing repeated parameters out of parameter dictionaries and
  into the main body of the for loop.
- We also compute unique, maximally-descriptive names for the tasks, a
  combination of the constraint name and the concept's position in the
  hierarchy. (e.g. `wine_table_has_replicated_schema`). These names are
  shortened as much as possible while still being unique (e.g., we may shorten
  things to `wine_schema`, a less mouthful of a task name).
- The driver then adds scaffolding for native Python, Airflow or Jupyter code
  generation. Other output formats (e.g. Prefect, Dagster, Makefiles, etc.)
  will be supported in the future.
- Finally, the driver converts the generated Python AST to a concrete string,
  which it then formats as a *pretty* (PEP8-compliant) Python program via
  Python [black](https://github.com/psf/black).

## Describing a dataset

Before we can turn our attention to what we would like to achieve with
our data, we need to determine what the data *is*, to begin with. We do
so via a dataset manifest, which is created using Python code.

Here's an example of how we'd create such a manifest for a canonical ML dataset
(the Wine dataset, as per `example/wine.py`).

First, we define our attribute list:
```python
attributes = [
    Categorical("wine_class_identifier"),
    PositiveFloat("alcohol"),
    PositiveFloat("malic_acid"),
    PositiveFloat("ash"),
    PositiveFloat("alcalinity_of_ash"),
    PositiveFloat("magnesium"),
    PositiveFloat("total_phenols"),
    PositiveFloat("non_flavanoid_phenols"),
    PositiveFloat("proanthocyanins"),
    PositiveFloat("color_intensity"),
    PositiveFloat("hue"),
    PositiveFloat("od_280__od_315_diluted_wines"),
    PositiveFloat("proline"),
]
```

Then, we express the fact that a row corresponds to a struct
with the fields defined in the `attributes` list:
```python
wine_datum = RowStruct(
    name="wine_datum",
    attributes=attributes,
)
```

Then, we declare that our data can be found somewhere on the Web, in
the `remote` storage. Note that we also record the data being CSV-encoded,
and the location corresponding to a single file. This is where we could
note compression algorithms, headers, etc.:
```python
remote = RemoteStorage(
    location=RemoteLocation(WebLocation(
        address=("https://archive.ics.uci.edu/ml/"
                 "machine-learning-databases/wine/wine.data"),
    )),
    layout=Layout(SingleFileLayout()),
    encoding=Encoding(CSVEncoding()),
)
```

We need this data to live locally, in a Hive table in ORC format, backed
by a MinIO location with the prefix `wine`:
```python
local = HiveTableStorage(
    location=Location(MinioLocation(name="wine")),
    layout=Layout(StaticHiveTableLayout()),
    encoding=Encoding(ORCEncoding()),
)
```
Note a few things:
- we don't specify the table name, as this is automatically-generated from the
  asset name (we will define that momentarily)
- we declare, "this thing needs to be stored in MinIO", but do not concern
  ourselves with endpoints at this moment. Aorist will find the right endpoints
  for us and fill in secrets, etc. Or if MinIO is unavailable it will fail.
- this is also where we can indicate whether our table is static (i.e. there is
  no time dimension, or dynamic).

We are now ready to define our asset, called `wine_table`:
```python
wine_table = StaticDataTable(
    name="wine_table",
    schema=default_tabular_schema(wine_datum),
    setup=StorageSetup(RemoteImportStorageSetup(
        tmp_dir="/tmp/wine",
        remote=remote,
        local=[local],
    )),
    tag="wine",
)
```
Here's what we do here:
  - we define an asset called `wine_table`. This is also going to be the name
  of any Hive table that will be created to back this asset (or file, or
  directory, etc., depending on the dataset storage).
  - we also define a schema. A schema tells us *exactly* how we can turn a row
  into a template. For instance, we need the exact order of columns in a row
  to know unambiguously how to convert it into a struct.
  - `default_tabular_schema` is a helper function that allows us to derive a
  schema where columnns in the table are in exactly the same order as fields in
  the struct.
  - the `setup` field introduces the notion of a "replicated" remote storage,
    via `RemoteImportStorageSetup`. The idea expressed here is that we should
    make sure the data available at the `remote` location is replicated exactly
    in the `local` locations (either by copying it over, or, if already
    availalbe, by checking that the remote and target data has the same
    checksum, etc.)
  - we also use a `tag` field to help generate legible task names and IDs
    (e.g., in Airflow)

Finally, let's define our dataset:

```python
wine_dataset = DataSet(
    name="wine",
    description="A DataSet about wine",
    sourcePath=__file__,
    datumTemplates=[DatumTemplate(wine_datum)],
    assets={"wine_table": Asset(wine_table)},
)
```
This dataset can then be imported into the universe discussed previously.

### Aside: The asset / template split

An Aorist dataset is meant to be a collection of two things:
- data *assets* -- concrete information, stored in one or multiple locations,
  remotely, on-premise, or in some sort of hybrid arrangement.
- datum *templates* -- information about what an instance of our data (i.e., a
  *datum*) represents.


For instance, a table is a data asset. It has rows and columns, and those rows
and columns are filled with some values that can be read from some location.

What those rows and columns *mean* depends on the template. Oftentimes rows in
tables translate to structs, for instance in a typical `dim_customers` table.
But if we're talking about graph data, then a row in our table represents a
tuple (more specifically a pair), and not a struct.

Other examples of data assets would be:
- directories with image files,
- concrete machine learning models,
- aggregations,
- scatterplots,

Other examples of data templates could be:
- a tensor data template corresponding to RGB images,
- an ML model template that takes a certain set of features (e.g. number of
  rooms and surface of a house, and produces a prediction, e.g. a valuation),
- a histogram data template, expressing the meaning of margin columns used for
  aggregations, as well as the aggregation function (a count for a histogram)
- a scatterplot template, encoding the meaning of the x and y axis, etc.

This conceptual differentiation allows us to use the same template to refer to
multiple assets. For instance, we may have multiple tables with exactly the
same schema, some being huge tables with real data, and others being
downsampled tables used for development. These tables should be refered to
using the same template.

This is also very useful in terms of tracking data lineage, on two levels:
semantically (how does template Y follow from template X?) and concretely (how
does row A in table T1 follow from row B in table T2?).

## Back to the SNAP dataset

The SNAP dataset we discussed initially is a bit different from the simple Wine
dataset. For one, it contains many assets -- this is a collection of different graphs
used for Machine Learning applications -- each graph is its own asset. But the
meaning of a row remains the same: it's a 2-tuple made up of identifiers. We
record this by defining the template:
```python
edge_tuple = IdentifierTuple(
    name="edge",
    attributes=[
        NumericIdentifier("from_id"),
        NumericIdentifier("to_id"),
    ],
)
```

Then we define an asset for each of 12 datasets. Note that the names come from
the URL patterns corresponding to each dataset. We need to replace dashes with
underscores when creating asset names however (Hive tables don't like dashes
in their names):
```python
names = [
    "ca-AstroPh", "ca-CondMat", "ca-GrQc", "ca-HepPh",
    "ca-HepTh", "web-BerkStan", "web-Google", "web-NotreDame",
    "web-Stanford", "amazon0302", "amazon0312", "amazon0505",
]
tables = {}
for name in names:

    name_underscore = name.replace("-", "_").lower()
    remote = Storage(RemoteStorage(
        location=Location(WebLocation(
            address="https://snap.stanford.edu/data/%s.txt.gz" % name,
        )),
        layout=Layout(SingleFileLayout()),
        encoding=Encoding(TSVEncoding(
            compression=Compression(GzipCompression()),
            header=FileHeader(UpperSnakeCaseCSVHeader(num_lines=4)),
        )),
    ))
    local = Storage(HiveTableStorage(
        location=Location(MinioLocation(name=name_underscore)),
        layout=Layout(StaticHiveTableLayout()),
        encoding=Encoding(ORCEncoding()),
    ))
    table = StaticDataTable(
        name=name_underscore,
        schema=default_tabular_schema(edge_tuple),
        setup=StorageSetup(RemoteImportStorageSetup(
            tmp_dir="/tmp/%s" % name_underscore,
            remote=remote,
            local=[local],
        )),
        tag=name_underscore,
    )
    tables[name] = table

snap_dataset = DataSet(
    name="snap",
    description="The Snap DataSet",
    sourcePath=__file__,
    datumTemplates=[edge_tuple],
    assets=tables,
    tag="snap",
)
```

## What if we want to do Machine Learning?

As a proof-of-concept, ML models are not substantively different from
tabular-based data assets. Here's an example for how we can declare the
existence of an SVM regression model trained on the wine table:

```python
# We will train a classifier and store it in a local file.
classifier_storage = Storage(LocalFileStorage(
    location=Location(MinioLocation(name="wine")),
    layout=Layout(SingleFileLayout()),
    encoding=Encoding(ONNXEncoding()),
))
# We will use these as the features in our classifier.
features = attributes[2:10]
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
    schema=classifier_template.get_model_storage_tabular_schema(),
    algorithm=Algorithm(SVMRegressionAlgorithm()),
)
wine_dataset.add_asset(regression_model)
```

Note the use of imperative directives such as `wine_dataset.add_asset`. This is
a small compromise on our mostly-declarative syntax, but it maps well on the
following thought pattern common to ML models:
- we have some "primary sources", datasets external to the project,
- we then derive other data assets by building, iteratively on the primary
  sources.

The common development cycle, therefore, is one where, after the original data
sources are imported, we add new templates and assets to our dataset,
fine-tuning Python code by first running it in Jupyter, then in Native python,
then as an Airflow task, etc.

Also note that while currently Aorist only supports generating single files as
DAGs, in the future we expect it will support multiple file generation for
complex projects.

