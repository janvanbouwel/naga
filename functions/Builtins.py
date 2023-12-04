import copy
from collections.abc import Callable

from Value import Value
from functions.Function import Function
from type.FunctionType import FT
from type.Generics import Generic, GenStack


def app_exec(stack: list[Value]):
    func = stack.pop()
    func.value(stack)


def app_func():
    gen_stack_a = GenStack("a")
    gen_stack_b = GenStack("b")

    return Function(FT(gen_stack_a.append(FT(gen_stack_a, gen_stack_b)), gen_stack_b), app_exec)


def id_func():
    gen = Generic("a")
    return Function(FT.new([gen], [gen]), lambda x: None)


def dup_exec(stack: list):
    stack.append(copy.deepcopy(stack[-1]))


def dup_func():
    gen = Generic("a")
    return Function(FT.new([gen], [gen, gen]), dup_exec)


builtin_functions: dict[str, Callable[[], Function]] = {
    "!": app_func,
    "id": id_func,
    "dup": dup_func
}


def parse(token: str) -> Function:
    if token in builtin_functions:
        return builtin_functions[token]()
