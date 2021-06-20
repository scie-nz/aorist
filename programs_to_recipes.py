import yaml
import re

with open('programs.yaml') as f:
    programs = list(yaml.safe_load_all(f))
programs = [x['spec'] for x in programs if x['spec']['dialect'] == 'Python']
programs = [x for x in programs if 'kwargs' in x]

def camel_to_snake(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

formatted = {}
for program in programs:
    name = program['use']
    args = program['kwargs']
    formatted[name] = """from aorist import aorist, {use}

programs = {{}}

@aorist(
    programs,
    {use},
    entrypoint="{call}",
    args={{
{args}
    }},
)
def recipe({arg_names}):
    {code}
    """.format(
        use=name,
        call=program["call"],
        arg_names=", ".join(args.keys()),
        args="\n".join(["        \"%s\": %s," % (
            k, 
            "lambda lng: lng.%s" % v["spec"]["call"].replace(".unwrap()","").replace(
                ".to_string()", "").replace(".as_ref()", "").replace(".clone()", "")
        ) for k, v in args.items()]),
        code=program["preamble"].replace("\n", "\n    "),
    )
    with open("recipes/%s.py" % camel_to_snake(name), 'w') as f:
        f.write(formatted[name])
