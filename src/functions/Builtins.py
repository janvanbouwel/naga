import copy

from src.Expression import Function
from src.functions.Numbers import add_function
from src.types.Types import Generic, FunctionType

gen_a = Generic("a")

builtin_functions = {
    "+": add_function,
    "dup": Function(FunctionType([gen_a], [gen_a, gen_a]), lambda x: [x[-1], copy.deepcopy(x[-1])])
}
