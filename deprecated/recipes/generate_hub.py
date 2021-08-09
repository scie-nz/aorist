from aorist import aorist, GenerateHub

programs = {}

@aorist(
    programs,
    GenerateHub,
    entrypoint="gen_hub_for_dataset",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "dataset_description": lambda lng: lng.data_set.description,
        "input_file": lambda lng: lng.data_set.sourcePath,
        "asset_names": lambda lng: lng.serde_json::json!(
    data_set.get_assets().iter().map(
        |x| (x.get_name(), x.get_type())
    ).collect::<Vec<_>>()
)
,
        "template_names": lambda lng: lng.serde_json::json!(
    data_set.get_templates().iter().map(
        |x| (x.get_name(), x.get_type())
    ).collect::<Vec<_>>()
)
,
    },
)
def recipe(dataset_name, dataset_description, input_file, asset_names, template_names):
    from datetime import datetime
    import json
    import os
    import yaml
    
    def gen_hub_for_dataset(
        dataset_name,
        dataset_description,
        input_file,
        asset_names,
        template_names,
    ):
        payload = {
            "name": dataset_name,
            "description": dataset_description,
            "input_file": input_file,
            "render_date": datetime.today().strftime('%Y-%m-%d'),
            "assets": json.loads(asset_names),
            "templates": json.loads(template_names),
        }
    
        base_dir = os.getenv('OUTPUT_DIR')
        if base_dir is None:
            raise Exception('Missing environment variable: OUTPUT_DIR')
        output_dir = os.path.join(base_dir, dataset_name)
    
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        with open(os.path.join(output_dir, '_index.yaml'), 'w') as f:
            yaml.dump(payload, f, indent=2)
    
    