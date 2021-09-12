from aorist import (
    RowStruct,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorageSetup,
    StaticDataTable,
    DataSet,
    default_tabular_schema,
    attr_list,
    GithubLocation,
    RemoteStorage,
    CSVHeader,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining dataset
"""
# Attributes in the dataset
attributes = attr_list([
    attr.FreeText("Date"),
    attr.StringIdentifier("Team 1"),
    attr.StringIdentifier("Score"),
    attr.StringIdentifier("Team 2"),
    attr.StringIdentifier("Tournament"),
    attr.FreeText("City"),
])
# A row is equivalent to a struct
cache_internationals_datum = RowStruct(
    name="cache_internationals_datum",
    attributes=attributes,
)
# Data can be found remotely, on the web
remote = RemoteStorage(
    location=GithubLocation(
        organization="footballcsv",
        repository="cache.internationals",
        path="1800s/1872.csv",
        branch="master",
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
)

# We will create a table that will always have the same content
# (we do not expect it to change over time)
year_1872_res = StaticDataTable(
    name="year_1872_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=remote,
    ),
    tag="1872_football",
)

year_1873_res = StaticDataTable(
    name="year_1873_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1873.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1873_football",
)

year_1874_res = StaticDataTable(
    name="year_1874_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1874.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1874_football",
)

year_1875_res = StaticDataTable(
    name="year_1875_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1875.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1875_football",
)

year_1876_res = StaticDataTable(
    name="year_1876_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1876.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1876_football",
)

year_1877_res = StaticDataTable(
    name="year_1877_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1877.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1877_football",
)

year_1878_res = StaticDataTable(
    name="year_1878_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1878.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1878_football",
)

year_1879_res = StaticDataTable(
    name="year_1879_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1879.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1879_football",
)

year_1880_res = StaticDataTable(
    name="year_1880_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1880.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1880_football",
)

year_1881_res = StaticDataTable(
    name="year_1881_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1881.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1881_football",
)

year_1882_res = StaticDataTable(
    name="year_1882_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1882.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1882_football",
)

year_1883_res = StaticDataTable(
    name="year_1883_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1883.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1883_football",
)

year_1884_res = StaticDataTable(
    name="year_1884_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1884.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1884_football",
)

year_1885_res = StaticDataTable(
    name="year_1885_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1885.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1885_football",
)

year_1886_res = StaticDataTable(
    name="year_1886_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1886.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1886_football",
)

year_1887_res = StaticDataTable(
    name="year_1887_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1887.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1887_football",
)

year_1888_res = StaticDataTable(
    name="year_1888_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1888.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1888_football",
)

year_1889_res = StaticDataTable(
    name="year_1889_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1889.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1889_football",
)

year_1890_res = StaticDataTable(
    name="year_1890_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1890.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1890_football",
)

year_1891_res = StaticDataTable(
    name="year_1891_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1891.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1891_football",
)

year_1892_res = StaticDataTable(
    name="year_1892_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1892.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1892_football",
)

year_1893_res = StaticDataTable(
    name="year_1893_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1893.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1893_football",
)

year_1894_res = StaticDataTable(
    name="year_1894_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1894.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1894_football",
)

year_1895_res = StaticDataTable(
    name="year_1895_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1895.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1895_football",
)

year_1896_res = StaticDataTable(
    name="year_1896_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1896.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1896_football",
)

year_1897_res = StaticDataTable(
    name="year_1897_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1897.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1897_football",
)

year_1898_res = StaticDataTable(
    name="year_1898_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1898.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1898_football",
)

year_1899_res = StaticDataTable(
    name="year_1899_res",
    schema=default_tabular_schema(cache_internationals_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=GithubLocation(
                organization="footballcsv",
                repository="cache.internationals",
                path="1800s/1899.csv",
                branch="master",
            ),
            layout=SingleFileLayout(),
            encoding=CSVEncoding(header=CSVHeader(num_lines=1)),
        )
    ),
    tag="1899_football",
)


# our dataset contains only this table and only this datum
# definition. Note that multiple assets can reference the
# same template!
int_1800_football_dataset = DataSet(
    name="int-1800-football",
    description=(
        "International Football Results from 1872-1899 (https://github.com/footballcsv/cache.internationals)"
    ),
    sourcePath=__file__,
    datumTemplates=[
    cache_internationals_datum,
    ],
    assets={
        "1872_international_football_results": year_1872_res,
        "1873_international_football_results": year_1873_res,
        "1874_international_football_results": year_1874_res,
        "1875_international_football_results": year_1875_res,
        "1876_international_football_results": year_1876_res,
        "1877_international_football_results": year_1877_res,
        "1878_international_football_results": year_1878_res,
        "1879_international_football_results": year_1879_res,
        "1880_international_football_results": year_1880_res,
        "1881_international_football_results": year_1881_res,
        "1882_international_football_results": year_1882_res,
        "1883_international_football_results": year_1883_res,
        "1884_international_football_results": year_1884_res,
        "1885_international_football_results": year_1885_res,
        "1886_international_football_results": year_1886_res,
        "1887_international_football_results": year_1887_res,
        "1888_international_football_results": year_1888_res,
        "1889_international_football_results": year_1889_res,
        "1890_international_football_results": year_1890_res,
        "1891_international_football_results": year_1891_res,
        "1892_international_football_results": year_1892_res,
        "1893_international_football_results": year_1893_res,
        "1894_international_football_results": year_1894_res,
        "1895_international_football_results": year_1895_res,
        "1896_international_football_results": year_1896_res,
        "1897_international_football_results": year_1897_res,
        "1898_international_football_results": year_1898_res,
        "1899_international_football_results": year_1899_res,
    },
)
