import yaml
import copy

template = """
type: DataSet
spec:
  tag: snap
  name: snap-dataset
  accessPolicies:
  - type: ApproveAccessSelector
    spec:
      matchLabels:
        department:
          - datascience
  datumTemplates:
  - name: edge
    type: IdentifierTuple
    attributes:
    - name: from_id
      type: NumericIdentifier
    - name: to_id
      type: NumericIdentifier
  assets:
    - type: StaticDataTable
      spec:
        tag: tag
        name: name
        setup:
          type: RemoteImportStorageSetup
          spec:
            tmp_dir: /tmp/wikitalk
            remote:
              type: RemoteStorage
              location:
                type: WebLocation
                spec:
                  address: https://snap.stanford.edu/data/ca-AstroPh.txt.gz
              layout:
                type: SingleFileLayout
              encoding:
                type: TSVEncoding
                spec:
                  compression:
                    type: GzipCompression
                  header:
                    type: UpperSnakeCaseCSVHeader
                    spec:
                      num_lines: 4
            local:
            - type: HiveTableStorage
              location:
                type: AlluxioLocation
                spec:
                  path: 'snap/ca-astroph'
              layout:
                type: StaticHiveTableLayout
                spec: {}
              encoding:
                type: ORCEncoding
                spec: {}
        schema:
          type: TabularSchema
          spec:
            datumTemplateName: edge
            attributes:
            - from_id
            - to_id
"""

doc = yaml.safe_load(template)
asset_templates = []
for name in [
    'ca-AstroPh', 'ca-CondMat', 'ca-GrQc', 'ca-HepPh', 'ca-HepTh',
    'web-BerkStan', 'web-Google', 'web-NotreDame', 'web-Stanford',
    'amazon0302', 'amazon0312', 'amazon0505', 'amazon0601',
]:
    t = copy.deepcopy(doc['spec']['assets'][0])
    t['spec']['tag'] = name.replace('-', '_')
    t['spec']['name'] = name.replace('-', '_')
    t['spec']['setup']['spec']['tmp_dir'] = "/tmp/snap/%s/" % name.replace('-', '_')
    t['spec']['setup']['spec']['remote']['location']['spec']['address'] = (
        "https://snap.stanford.edu/data/%s.txt.gz" % name
    )
    asset_templates += [t]
doc['spec']['assets'] = asset_templates
print(yaml.dump(doc))
