import copy

from src.functions.Function import Function
from src.types.Types import Generic, FunctionType

gen_a = Generic("a")

builtin_functions = {
    "dup": Function(FunctionType([gen_a], [gen_a, gen_a]), lambda x: [x[-1], copy.deepcopy(x[-1])])
}


def parse(token: str):
    if token in builtin_functions:
        return builtin_functions[token]
