import inspect
import dill
from aorist.target.debug.libaorist import *

def to_str(source):
    funcString = "\n".join(inspect.getsourcelines(source)[0])
    print(funcString)
    return funcString


def aorist(programs, constraint, entrypoint, args, pip_requirements=[]):
    args_str = {
        k : (
            list(inspect.signature(v).parameters.keys()),
            dill.dumps(lambda x: v(*x)).decode('latin-1')
        ) for k, v in args.items()
    }
    print(args_str)
    def inner(func):
        programs[constraint] = constraint.register_python_program(
            to_str(func), 
            entrypoint, 
            [], 
            args_str, 
            pip_requirements
        )
    return inner

