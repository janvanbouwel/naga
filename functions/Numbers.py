from collections.abc import Callable

from Value import Value
from functions.Function import Function
from type.FunctionType import FT
from type.Types import BaseType

Int = BaseType("Int")


def int_value(value: int):
    return Value(Int, value)


def int_literal(value: int):
    return Function(FT.new([], [Int]), lambda stack: stack.append(int_value(value)))


def create_op_func(op: Callable[[int, int], int]):
    def op_func(stack):
        [x, y] = stack[-2:]
        del stack[-2:]
        stack.append(int_value(op(x.value, y.value)))

    return Function(MathFunctionType, op_func)


MathFunctionType = FT.new([Int, Int], [Int])

builtin_functions = {
    "+": create_op_func(lambda x, y: x + y),
    "-": create_op_func(lambda x, y: x - y),
    "*": create_op_func(lambda x, y: x * y),
    "/": create_op_func(lambda x, y: x / y)}


def parse(token: str):
    if token in builtin_functions:
        return builtin_functions[token]
    try:
        return int_literal(int(token))
    except ValueError:
        pass
