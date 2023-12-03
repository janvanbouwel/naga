from src.functions.Function import Function
from src.Value import Value
from src.types.Types import FunctionType, BaseType

Int = BaseType("Int")


def int_literal(value: int):
    return Function(FunctionType([], [Int]), lambda _: [Value(Int, value)])


def add(x: Value, y: Value):
    return [Value(Int, x.value + y.value)]


MathFunctionType = FunctionType([Int, Int], [Int])

add_function = Function(MathFunctionType, lambda x: add(*x))

builtin_functions = {
    "+": add_function
}


def parse(token: str):
    if token in builtin_functions:
        return builtin_functions[token]
    try:
        return int_literal(int(token))
    except ValueError:
        pass
