from aorist import (
    User,
    EndpointConfig,
    AlluxioConfig,
    PrestoConfig,
    RangerConfig,
    GiteaConfig,
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
endpoint_config = EndpointConfig(
    alluxio=alluxio_config,
    ranger=ranger_config,
    presto=presto_config,
    gitea=gitea_config,
)

"""
Defining users.
"""
bogdan = User(
    firstName="Bogdan", lastName="State", email="bogdan@scie.nz", unixname="bogdan"
)
nick = User(firstName="Nick", lastName="Parker", email="nick@scie.nz", unixname="nick")
cip = User(firstName="Ciprian", lastName="Gerea", email="cip@scie.nz", unixname="cip")
