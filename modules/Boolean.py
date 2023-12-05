from collections.abc import Callable
from enum import Enum

from functions.Function import Function
from functions.Module import Module
from functions.Symbol import Symbol
from type.InstructionType import FT
from type.Generics import Generic
from type.Types import BaseType

Bool = BaseType("Bool")


class BOOL(Enum):
    TRUE = 1
    FALSE = 2


def bool_literal(value: bool):
    return Function.new([FT.new([], [Bool])], lambda stack: stack.append(cast(value)))


def cast(x):
    return BOOL.TRUE if x else BOOL.FALSE


def if_exec(stack):
    [cond, t, f] = stack[-3:]
    del stack[-3:]

    stack.append(t if cond == BOOL.TRUE else f)


def if_func():
    gen = Generic("a")
    return Function.new([FT.new([Bool, gen, gen], [gen])], if_exec)


def eq_exec(stack: list):
    stack.append(cast(stack.pop() == stack.pop()))


def eq_func():
    gen = Generic("a")
    return Function.new([FT.new([gen, gen], [Bool])], eq_exec)


class Boolean(Module):

    @staticmethod
    def parse(token: str) -> Symbol:
        if token in ["TRUE", "FALSE"]:
            return bool_literal(token == "TRUE")

    @staticmethod
    def built_in() -> dict[str, Callable[[], Function]]:
        return {
            "?": if_func,
            "=": eq_func
        }
