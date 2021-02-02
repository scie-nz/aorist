import aorist
from aorist import User, EndpointConfig, AlluxioConfig, RangerConfig

alluxio_config = AlluxioConfig(server='alluxio')
ranger_config = RangerConfig(server='localhost', user='admin', password='G0powerRangers')

print(aorist.build_from_yaml('basic.yaml'))
