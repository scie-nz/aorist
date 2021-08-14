
## SQL and derived assets

Especially when datasets are in tabular form, it makes sense to think of data
transformations in terms of standard SQL operations -- selections, projections,
groups, explodes, and joins. These transformations can be supported via a
`derive_asset` directive used in the process of Universe creation. For
instance, if we are interested in training a model for high-ABV wines only, we
can write:

```python
universe.derive_asset(
    """
    SELECT *
    FROM wine.wine_table
    WHERE wine.wine_table.alcohol > 14.0
    """,
    name="high_abv_wines",
    storage=HiveTableStorage(
        location=MinioLocation(name="high_abv_wines"),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    ),
    tmp_dir="/tmp/high_abv_wines",
)
```

Behind the scenes, this directive does two things:
- if necessary, creates a new template expressing the operation of filtering a
  table on the alcohol attribute.
- it creates a new `StaticDataTable` asset living in the indicated storage.
  This table will only be computed *after* its source tables (the ones in the
  `FROM` clause) are ready.


