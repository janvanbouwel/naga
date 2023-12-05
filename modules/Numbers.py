from collections.abc import Callable

from functions.Function import Function
from functions.Module import Module
from functions.Symbol import Symbol
from type.InstructionType import FT
from type.Types import BaseType

Int = BaseType("Int")


def int_value(value: int):
    return value


def extract_value(value) -> int:
    return value


def int_literal(value: int):
    return Function.new([FT.new([], [Int])], lambda stack: stack.append(int_value(value)))


def create_op_func(op: Callable[[int, int], int]):
    def op_func(stack):
        [x, y] = stack[-2:]
        del stack[-2:]
        stack.append(int_value(op(extract_value(x), extract_value(y))))

    return Function.new([MathFunctionType], op_func)


MathFunctionType = FT.new([Int, Int], [Int])


class Number(Module):
    @staticmethod
    def parse(token: str) -> Symbol:
        try:
            return int_literal(int(token))
        except ValueError:
            pass

    @staticmethod
    def built_in() -> dict[str, Callable[[], Function]]:
        return {
            "+": lambda: create_op_func(lambda x, y: x + y),
            "-": lambda: create_op_func(lambda x, y: x - y),
            "*": lambda: create_op_func(lambda x, y: x * y),
            "/": lambda: create_op_func(lambda x, y: x / y)}
