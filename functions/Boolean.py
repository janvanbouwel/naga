from Value import Value
from functions.Function import Function
from type.FunctionType import FunctionType
from type.Types import BaseType, Generic

Bool = BaseType("Bool")

TRUE = Value(Bool, True)
FALSE = Value(Bool, False)


def bool_literal(value: bool):
    return Function(FunctionType([], [Bool]), lambda _: [cast(value)])


def cast(x):
    return TRUE if x else FALSE


gen_a = Generic("a")


def equal_function(x: Value, y: Value):
    return [cast(x == y)]


builtin_functions = {
    "=": Function(FunctionType([gen_a, gen_a], [Bool]), lambda x: equal_function(*x)),
}


def parse(token: str):
    if token in ["TRUE", "FALSE"]:
        return bool_literal(token == "TRUE")
    if token in builtin_functions:
        return builtin_functions[token]
