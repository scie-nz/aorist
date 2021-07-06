import inspect
import dill
import ast
import astor
from aorist.target.debug.libaorist import *
import re
import imp
import builtins

def default_tabular_schema(datum, template_name, attributes):
    return DataSchema(TabularSchema(
        datumTemplateName=template_name,
        attributes=[a.name for a in attributes],
    ))


def to_str(source):
    raw = "\n".join(inspect.getsourcelines(source)[0])
    parsed = ast.parse(raw)
    funcString = astor.to_source(ast.Module(parsed.body[0].body))
    return funcString


def aorist(programs, constraint, entrypoint, args, pip_requirements=[]):
    args_str = {
        k : (
            list(inspect.signature(v).parameters.keys()),
            dill.dumps(lambda x: v(*x)).decode('latin-1')
        ) for k, v in args.items()
    }
    def inner(func):
        programs[constraint] = constraint.register_python_program(
            to_str(func),
            entrypoint,
            [],
            args_str,
            pip_requirements
        )
    return inner

def aorist_presto(programs, constraint, entrypoint, args):
    args_str = {
        k : (
            list(inspect.signature(v).parameters.keys()),
            dill.dumps(lambda x: v(*x)).decode('latin-1')
        ) for k, v in args.items()
    }
    programs[constraint] = constraint.register_presto_program(
        "",
        entrypoint,
        [],
        args_str,
    )

def aorist_bash(programs, constraint, entrypoint, args):
    args_str = {
        k : (
            list(inspect.signature(v).parameters.keys()),
            dill.dumps(lambda x: v(*x)).decode('latin-1')
        ) for k, v in args.items()
    }
    def inner(func):
        programs[constraint] = constraint.register_bash_program(
            to_str(func),
            entrypoint,
            [],
            args_str,
        )
    return inner

def sql_module(filename):
    text = open(filename).read()
    assert(text.index('/***') == 0)
    b = text.index('***/')
    decorator = text[5:b]
    program = decorator[1:]
    entrypoint = text[b + 4:].strip()

    tree = ast.parse(program)
    assert(len(tree.body[0].value.args) == 2)
    constraint = tree.body[0].value.args[1]
    tree.body[0].value.args += [ast.Constant(entrypoint)]
    tree.body.insert(0, ast.Import(
        [ast.alias(name="builtins", asname=None)],
    ))
    tree.body.insert(0, ast.ImportFrom("aorist", [
        ast.alias(name="aorist_presto", asname=None),
        ast.alias(name=constraint.id, asname=None),
    ], 0))
    tree.body.insert(2, ast.Assign(
        [ast.Attribute(
            ast.Name(id="builtins", ctx=ast.Load()),
            constraint.id,
            ast.Store(),
        )],
        ast.Name(id=constraint.id, ctx=ast.Load()),
    ))
    tree.body.insert(3, ast.Assign(
        [ast.Name(id="programs", ctx=ast.Store())],
        ast.Dict([],[]),
    ))
    code = astor.to_source(tree)
    module_name = filename.replace('presto.sql', '').replace('recipes/', '')
    module = imp.new_module(module_name)
    exec(code, module.__dict__)
    return module

