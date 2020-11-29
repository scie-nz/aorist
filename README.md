# dataorist

## Concepts
Aorist uses two types of concepts:
- abstract concepts (e.g. a "location") 
- concrete ones (e.g. "a Google Cloud Storage location", or `GCSLocation`).

The relationship between abstract concepts represents the core semantic model offered by Aorist. This is not expected to change on a regular basis. Ideally this would not change at all.

Concrete concepts "instantiate" abstract ones, much like classes can instantiate traits or interfaces in OOP languages (in fact, this is how concrete concepts are implemented in Aorist).

Abstract concepts have the following hierarchy:

- `DataSetup`: the core Aorist abstraction, one per project
  - `DataSet`:  a universe of instantiated objects which have inter-related schemas
  - `User`:  someone accessing the objects
  - `Group`:  a group of users
  - `Roles`:  a way in which a user may access data.
  - `RoleBindings`:  a connection between users and roles 

## Constraints

A constraint is a fact that can be verified about an abstract concept. A constraint may have dependent constraints. For instance, we may have the constraint "is consistent" on `DataSetup`, that further breaks down into:
- "datasets are replicated",
- "users are instantiated",
- "role bindings are created", 
- "data access policies are enforced".

Dependent constraints simply tell us what needs to hold, in order for a constraint to be true. Dependent constraints may be defined on the same concept, or on dependent concepts.
# charmed-labs-vizy
