# Aorist

Aorist is a code-generation tool for MLOps. Its aim is to generate legible
code for common repetitive tasks in data science, such as data replication,
common transformations, as well as machine learning operations.

## How to install

These instructions require [Anaconda](https://anaconda.org) and were tested
against Ubuntu Linux 20.04 LTS.

```
conda create -n aorist python=3.8 anaconda
conda activate aorist
pip install https://storage.googleapis.com/scienz-artifacts/aorist-0.0.1-cp38-cp38-manylinux2010_x86_64.whl

# Try it out
python example/gen.py python
python example/gen.py jupyter
python example/gen.py airflow
```

## Overview of an Aorist universe

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

```
DIALECT = "python"
out = dag(
  universe, [
    "DataDownloadedAndConverted",
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
```
DIALECT = "jupyter"
```
Or we can set `DIALECT` to `"airflow"` for an Airflow DAG.




## How to build

To build cargo library (need Rust installed):

```
cargo build
```

To try out Python code against .so library:

```
./run.sh
```

To rebuild pip wheel (requires [maturin](https://github.com/PyO3/maturin)):
```
maturin build
```

## Concepts
Aorist uses two types of concepts:
- abstract concepts (e.g. a "location")
- concrete ones (e.g. "a Google Cloud Storage location", or `GCSLocation`).

The relationship between abstract concepts represents the core semantic model offered by Aorist. This is not expected to change on a regular basis. Ideally this would not change at all.

Concrete concepts "instantiate" abstract ones, much like classes can instantiate traits or interfaces in OOP languages (in fact, this is how concrete concepts are implemented in Aorist).

Abstract concepts have the following hierarchy:

- `Universe`: the core Aorist abstraction, one per project
  - `DataSet`:  a universe of instantiated objects which have inter-related schemas
  - `User`:  someone accessing the objects
  - `Group`:  a group of users
  - `Roles`:  a way in which a user may access data.
  - `RoleBindings`:  a connection between users and roles

Here is the current hierarchy of Aorist concepts:

![Hierarchy of Aorist Concepts](./aorist_constrainables.svg)

## Constraints

A constraint is a fact that can be verified about a concept.
A constraint may have dependent constraints. For instance, we may have
the constraint "is consistent" on `Universe`, that further breaks down into:
- "datasets are replicated",
- "users are instantiated",
- "role bindings are created",
- "data access policies are enforced".

Dependent constraints simply tell us what needs to hold, in order for a constraint
to be true. Dependent constraints may be defined on the same concept, on
dependent concepts, or on higher-order concepts, but they may not create a
cycle. So we cannot say that constraint A depends on B, B depends on C, and C
depends on A.

This is quite dry stuff. Here is a diagram of an example set of constraints to
help better visualize what's going on:

![Hierarchy of Aorist Concepts and Constraints](./aorist_constrainables_with_constraints.svg)


When dependent constraints are defined on lower-order concepts, we will consider
the dependency to be satisfied when *ALL* constraints of the dependent kind
associated with the lower-order concepts directly inheriting from the
constrained concept have been satisfied.

For instance we may say that a constraint placed on the Universe (our
abstraction for a Data Warehouse or Data Lake), of the kind: "no columns
contain PII" is to be satisfied when all columns in *ALL* the tables are
confirmed to not contain any PII.

When dependent constraints are defined on higher-order concepts, we will
consider the dependency to be satisfied when the dependent constraint placed on
the exact higher-order ancestor has been satisfied.

So for instance, a model trained on data from the data warehouse may be
publishable on the web if we can confirm that no data in the warehouse
whatsoever contains any PII. This is a very strict guarantee, but it is
logically correct -- if there is no PII in the warehouse, there can be no PII
in the model. This is why we could have a constraint at the model-level that
depends on the Universe-level "no PII" constraint.

## Constraint DAG generation

Both constraints and concept operate at a very abstract level. They are basic
semantic building blocks of how we understand the things we care about in our
data streams. But our YAML file will define ``instances'' of concepts, i.e.,
Aorist **objects**. `StaticDataTable` is a concept, but we may have 200 static
data tables, on which we would like to impose the same constraints. For
instance, we would like all these tables to be audited, etc.[1]

Looking back at the concept hierarchy mentioned above, we turn the constraint
DAG into the prototype of an ETL pipeline by "walking" both the concept
(black) and constraint (red) part of the DAG.

Here's what the Constraint DAG looks like

![Constraint DAG](./aorist_dag.svg)

[1] (NOTE: in the future we will support filters on constraints, but for now
assume that all constraints must hold for all instances).

Some things to note about this DAG:
- it includes some superfluous dependencies, such as the one between
`DownloadDataFromRemote` and `ReplicatedData`
- some constraints are purely "cosmetic" -- `DataFromRemoteDownloaded` is
  really just a wrapper around `DownloadDataFromRemote` that "elevates" it to
  the root level, so that `UploadDataToLocal` can depend on it.

## Programs

Constraints are satisfiable in two stages:
1. First, any dependent constraints must be satisfied.
2. Then, a program associated with the constraint must run successfully.

The program is where the actual data manipulation happens. Examples of programs
are: "move this data from A to B", or "train this model", or "anonymize this
data," etc. The programs are written as templates, with full access to the
instantiated object hierarchy.

A program is written in a "dialect" that encompases what is considered to be
valid code. For instance, "Python3 with numpy and PyTorch" would be a dialect.
For Python dialects, we may attach a conda `requirements.txt` file, or a Docker
image to the dialect, etc. For R dialects we may attach a list of libraries and
an R version, or a Docker image.

## Drivers

Note that multiple programs may exist that could technically satisfy a
constraint. A **driver** decides which program to apply (given a preference
ordering) and is responsible for instantiating it into valid code that will run
in the specific deployment. A driver could, for instance, be responsible for
translating the constraint graph into valid Airflow code that will run in a
particular data deployment, etc.
# aorist-dags
