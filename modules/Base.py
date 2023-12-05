import copy
from collections.abc import Callable

from functions.Function import Function
from functions.Module import Module
from functions.Symbol import Symbol
from type.InstructionType import FT
from type.Generics import Generic, GenericStack, GenericFunction


def app_exec(stack: list):
    func = stack.pop()
    func(stack)


def app_func():
    func = GenericFunction.new("a")

    return Function.new([FT(func.in_type.append(func), func.out_type)], app_exec)


def id_func():
    gen = Generic("a")
    return Function.new([FT.new([gen], [gen])], lambda x: None)


def dup_exec(stack: list):
    stack.append(copy.deepcopy(stack[-1]))


def dup_func():
    gen = Generic("a")
    return Function.new([FT.new([gen], [gen, gen])], dup_exec)


class Base(Module):

    @staticmethod
    def parse(token: str) -> Symbol:
        pass

    @staticmethod
    def built_in() -> dict[str, Callable[[], Function]]:
        return {
            "!": app_func,
            "id": id_func,
            "dup": dup_func,
            "nop": lambda: Function.new([FT.new([], [])], lambda x: x)
        }
