import copy

from functions.Function import Function
from type.FunctionType import FunctionType
from type.Types import Generic

gen_a = Generic("a")

id_function = Function(FunctionType([gen_a], [gen_a]), lambda x: x)

builtin_functions = {
    "id": id_function,
    "dup": Function(FunctionType([gen_a], [gen_a, gen_a]), lambda x: [x[-1], copy.deepcopy(x[-1])])
}


def parse(token: str) -> Function:
    if token in builtin_functions:
        return builtin_functions[token]
