# Aorist

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
