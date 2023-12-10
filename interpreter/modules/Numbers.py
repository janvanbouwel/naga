from collections.abc import Callable

from functions.Function import Function
from functions.Module import Module
from type.InstructionType import FT
from type.Types import BaseType

Number = BaseType("Number")


def float_value(value: float):
    return value


def extract_value(value) -> float:
    return value


def number_literal(text: bytes):
    value = float(text.decode())
    return Function.new([FT.new([], [Number])], lambda stack: stack.append(float_value(value)))


def create_op_func(op: Callable[[float, float], float]):
    def op_func(stack):
        [x, y] = stack[-2:]
        del stack[-2:]
        stack.append(float_value(op(extract_value(x), extract_value(y))))

    return Function.new([MathFunctionType], op_func)


MathFunctionType = FT.new([Number, Number], [Number])


class Numbers(Module):

    @staticmethod
    def built_in() -> dict[str, Callable[[], Function]]:
        return {
            "+": lambda: create_op_func(lambda x, y: x + y),
            "-": lambda: create_op_func(lambda x, y: x - y),
            "*": lambda: create_op_func(lambda x, y: x * y),
            "/": lambda: create_op_func(lambda x, y: x / y)}
