from aorist import (
    User,
    EndpointConfig,
    AlluxioConfig,
    PrestoConfig,
    RangerConfig,
    GiteaConfig,
    UserGroup,
    GlobalPermissionsAdmin,
)

"""
Defining endpoints.
"""
alluxio_config = AlluxioConfig(server="alluxio")
ranger_config = RangerConfig(
    server="localhost", user="admin", password="G0powerRangers"
)
presto_config = PrestoConfig(server="presto-coordinator-0")
gitea_config = GiteaConfig(token="2b44b07e042ee9fe374e3eeebd2c9098468b5774")
endpoints = EndpointConfig(
    alluxio=alluxio_config,
    ranger=ranger_config,
    presto=presto_config,
    gitea=gitea_config,
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
