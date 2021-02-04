from aorist import (
    airflow_dag,
    Universe,
)
from common import (DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS)
from sentinel import sentinel_dataset

universe = Universe(
    name='my_cluster',
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    datasets=[sentinel_dataset],
    endpoints=DEFAULT_ENDPOINTS,
)
dag = airflow_dag(universe)
print(dag)
