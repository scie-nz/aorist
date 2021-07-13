import inspect
import dill
import ast
import astor
from aorist.target.debug.libaorist import *
import re
import imp
import builtins
from functools import wraps
import linecache
import collections
import os


def default_tabular_schema(datum, template_name, attributes):
    return DataSchema(TabularSchema(
        datumTemplateName=template_name,
        attributes=[a.name for a in attributes],
    ))


def to_str(source):
    source_lines, _ = inspect.getsourcelines(source)
    
    f = inspect.getsourcefile(source)
    linecache.checkcache(f)
    module = inspect.getmodule(source, f)
    lines = linecache.getlines(f, module.__dict__)
    func = source.__code__
    lnum = func.co_firstlineno - 1
    pat_decorator = re.compile(
        r'^(\s*@aorist)'
    )
    pat_recipe = re.compile(
        r'^(\s*def recipe)'
    )
    decorator_pos = [i for (i, l) in enumerate(lines) if re.match(pat_decorator, l)]
    recipe_pos = [i for (i, l) in enumerate(lines) if re.match(pat_recipe, l)]
    if len(decorator_pos) != 1 or len(recipe_pos) != 1:  
        raise ValueError("recipe function must be unique and last in file")
    source_lines = lines[decorator_pos[0]:]

    raw = "\n".join(source_lines)
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
        @wraps(func)
        def inner_func(func):
            programs[constraint] = constraint.register_python_program(
                to_str(func),
                entrypoint,
                [],
                args_str,
                pip_requirements
            )
        return inner_func(func)
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
    programs[constraint] = constraint.register_bash_program(
        "",
        entrypoint,
        [],
        args_str,
    )

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

def bash_module(filename):
    text = open(filename).read()
    assert(text.index('###+') == 0)
    b = text[1:].index('###+')
    decorator = text[5:b].replace("# ", "")
    program = decorator[1:]
    entrypoint = text[b + 5:].strip()

    tree = ast.parse(program)
    assert(len(tree.body[0].value.args) == 2)
    constraint = tree.body[0].value.args[1]
    tree.body[0].value.args += [ast.Constant(entrypoint)]
    tree.body.insert(0, ast.Import(
        [ast.alias(name="builtins", asname=None)],
    ))
    tree.body.insert(0, ast.ImportFrom("aorist", [
        ast.alias(name="aorist_bash", asname=None),
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
    module_name = filename.replace('.sh', '').replace('recipes/', '')
    module = imp.new_module(module_name)
    exec(code, module.__dict__)
    return module


def register_recipes(py_modules=[], bash_modules=[], sql_modules=[]):
    programs = collections.defaultdict(lambda: [])
    for module in py_modules:
        for k, v in module.programs.items():
            programs[k.name] += [AoristConstraintProgram(v)]
    for bash_module_name in bash_modules:
        module = bash_module(bash_module_name)
        for k, v in module.programs.items():
            programs[k.name] += [AoristConstraintProgram(v)]
    for sql_module_name in sql_modules:
        module = sql_module(sql_module_name)
        for k, v in module.programs.items():
            programs[k.name] += [AoristConstraintProgram(v)]
    return dict(programs)


