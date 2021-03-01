from aorist import (
    User,
    EndpointConfig,
    AlluxioConfig,
    PrestoConfig,
    RangerConfig,
    GiteaConfig,
    MinioConfig,
    UserGroup,
    GlobalPermissionsAdmin,
)

"""
Defining endpoints.
"""
alluxio_config = AlluxioConfig(
    server="alluxio-server",
    server_cli="alluxio-server",
    rpcPort=19998,
    apiPort=39999,
    directory="data",
)
ranger_config = RangerConfig(
    server="localhost", user="admin", password="G0powerRangers"
)
presto_config = PrestoConfig(server="trino-coordinator-headless", user="bogdan")
gitea_config = GiteaConfig(token="2b44b07e042ee9fe374e3eeebd2c9098468b5774")
minio_config = MinioConfig(
    server="minio",
    port=9000,
    bucket="minio-test-bucket",
    access_key="cppBrbSkEg5Vet6Mb0D4",
    secret_key="eRtRoywXqKBj0yHDyIaYb0c1Xnr5A3mCGsiT67Y1",
)
endpoints = EndpointConfig(
    alluxio=alluxio_config,
    ranger=ranger_config,
    presto=presto_config,
    gitea=gitea_config,
    minio=minio_config,
)

"""
Defining roles
"""
global_permissions_admin = GlobalPermissionsAdmin()

"""
Defining users.
"""
bogdan = User(
    firstName="Bogdan",
    lastName="State",
    email="bogdan@scie.nz",
    unixname="bogdan",
    roles=[global_permissions_admin],
)
nick = User(firstName="Nick", lastName="Parker", email="nick@scie.nz", unixname="nick")
cip = User(firstName="Ciprian", lastName="Gerea", email="cip@scie.nz", unixname="cip")

"""
Defining user groups
"""

finance = UserGroup(
    name="finance-users", users=[bogdan], labels={"department": "finance"}
)
datascience = UserGroup(
    name="finance-users",
    users=[bogdan, nick, cip],
    labels={"department": "datascience"},
)
crowding = UserGroup(
    name="project-crowding-detection",
    users=[bogdan],
    labels={"project": "crowding_detection"},
)

DEFAULT_USERS = [bogdan, nick, cip]
DEFAULT_GROUPS = [finance, datascience, crowding]
DEFAULT_ENDPOINTS = endpoints
