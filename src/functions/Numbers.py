from src.Expression import Function
from src.Value import Value
from src.types.Types import FunctionType, BaseTypes


def int_literal(value: int):
    return Function(FunctionType([], [BaseTypes.Int]), lambda _: [Value(BaseTypes.Int, value)])


def add(x: Value, y: Value):
    return [Value(BaseTypes.Int, x.value + y.value)]


MathFunctionType = FunctionType([BaseTypes.Int, BaseTypes.Int], [BaseTypes.Int])

add_function = Function(MathFunctionType, lambda x: add(*x))
