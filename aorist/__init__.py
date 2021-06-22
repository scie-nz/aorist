import inspect
from aorist.target.debug.libaorist import PushshiftAPILocation
from aorist_constraint.target.debug.libaorist_constraint import *

def to_str(source):
    funcString = "\n".join([str(x) for x in inspect.getsourcelines(source)])
    return funcString


def aorist(programs, constraint, entrypoint, args):
    args_str = {k : to_str(v) for k, v in args.items()}
    def inner(func):
        programs[constraint] = constraint.register_program(to_str(func), entrypoint, args_str)
    return inner

