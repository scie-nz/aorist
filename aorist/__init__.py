import inspect
import dill
import ast
import astor
from aorist.target.debug.libaorist import *

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
    def inner(func):
        programs[constraint] = constraint.register_python_program(
            to_str(func),
            entrypoint,
            [],
            args_str,
            pip_requirements
        )
    return inner
