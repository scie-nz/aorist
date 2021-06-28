import inspect
from aorist.target.debug.libaorist import *

def to_str(source):
    funcString = inspect.getsourcelines(source)[0][0]
    return funcString


def aorist(programs, constraint, entrypoint, args, pip_requirements=[]):
    args_str = {
        k : (
            list(inspect.signature(v).parameters.keys()),
            to_str(v)
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

