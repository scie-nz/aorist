"""
Description taken from: https://archive.ics.uci.edu/ml/datasets/wine

1. Title of Database: Wine recognition data
    Updated Sept 21, 1998 by C.Blake : Added attribute information

2. Sources:
   (a) Forina, M. et al, PARVUS - An Extendible Package for Data
       Exploration, Classification and Correlation. Institute of Pharmaceutical
       and Food Analysis and Technologies, Via Brigata Salerno,
       16147 Genoa, Italy.

   (b) Stefan Aeberhard, email: stefan@coral.cs.jcu.edu.au
   (c) July 1991
3. Past Usage:

   (1)
   S. Aeberhard, D. Coomans and O. de Vel,
   Comparison of Classifiers in High Dimensional Settings,
   Tech. Rep. no. 92-02, (1992), Dept. of Computer Science and Dept. of
   Mathematics and Statistics, James Cook University of North Queensland.
   (Also submitted to Technometrics).

   The data was used with many others for comparing various
   classifiers. The classes are separable, though only RDA
   has achieved 100% correct classification.
   (RDA : 100%, QDA 99.4%, LDA 98.9%, 1NN 96.1% (z-transformed data))
   (All results using the leave-one-out technique)

   In a classification context, this is a well posed problem
   with "well behaved" class structures. A good data set
   for first testing of a new classifier, but not very
   challenging.

   (2)
   S. Aeberhard, D. Coomans and O. de Vel,
   "THE CLASSIFICATION PERFORMANCE OF RDA"
   Tech. Rep. no. 92-01, (1992), Dept. of Computer Science and Dept. of
   Mathematics and Statistics, James Cook University of North Queensland.
   (Also submitted to Journal of Chemometrics).

   Here, the data was used to illustrate the superior performance of
   the use of a new appreciation function with RDA.

4. Relevant Information:

   -- These data are the results of a chemical analysis of
      wines grown in the same region in Italy but derived from three
      different cultivars.
      The analysis determined the quantities of 13 constituents
      found in each of the three types of wines.

   -- I think that the initial data set had around 30 variables, but
      for some reason I only have the 13 dimensional version.
      I had a list of what the 30 or so variables were, but a.)
      I lost it, and b.), I would not know which 13 variables
      are included in the set.

   -- The attributes are (dontated by Riccardo Leardi,
    riclea@anchem.unige.it )
     1) Alcohol
     2) Malic acid
     3) Ash
    4) Alcalinity of ash
     5) Magnesium
    6) Total phenols
     7) Flavanoids
     8) Nonflavanoid phenols
     9) Proanthocyanins
    10)Color intensity
     11)Hue
     12)OD280/OD315 of diluted wines
     13)Proline

5. Number of Instances

          class 1 59
    class 2 71
    class 3 48

6. Number of Attributes

    13

7. For Each Attribute:

    All attributes are continuous

    No statistics available, but suggest to standardise
    variables for certain uses (e.g. for us with classifiers
    which are NOT scale invariant)

    NOTE: 1st attribute is class identifier (1-3)

8. Missing Attribute Values:

    None

9. Class Distribution: number of instances per class

    class 1 59
    class 2 71
    class 3 48
"""

from aorist import (
    KeyedStruct,
    MinioLocation,
    WebLocation,
    StaticHiveTableLayout,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteImportStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining dataset
"""
attributes = [
    attr.Categorical("wine_class_identifier"),
    attr.PositiveFloat("alcohol"),
    attr.PositiveFloat("malic_acid"),
    attr.PositiveFloat("ash"),
    attr.PositiveFloat("alcalinity_of_ash"),
    attr.PositiveFloat("magnesium"),
    attr.PositiveFloat("total_phenols"),
    attr.PositiveFloat("non_flavanoid_phenols"),
    attr.PositiveFloat("proanthocyanins"),
    attr.PositiveFloat("color_intensity"),
    attr.PositiveFloat("hue"),
    attr.PositiveFloat("od_280__od_315_diluted_wines"),
    attr.PositiveFloat("proline"),
]
wine_datum = KeyedStruct(
    name="wine_datum",
    attributes=attributes,
)
remote = RemoteStorage(
    location=WebLocation(
        address=("https://archive.ics.uci.edu/ml/"
                 "machine-learning-databases/wine/wine.data"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(),
)
local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
wine_table = StaticDataTable(
    name="wine_table",
    schema=default_tabular_schema(wine_datum),
    setup=RemoteImportStorageSetup(
        tmp_dir="/tmp/wine",
        remote=remote,
        local=[local],
    ),
    tag="wine",
)
wine_dataset = DataSet(
    name="wine",
    datumTemplates=[wine_datum],
    assets=[wine_table],
)
