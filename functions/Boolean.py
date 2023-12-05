from collections.abc import Callable

from Value import Value
from functions.Function import Function
from type.FunctionType import FT
from type.Generics import Generic
from type.Types import BaseType

Bool = BaseType("Bool")

TRUE = Value(Bool, True)
FALSE = Value(Bool, False)


def bool_literal(value: bool):
    return Function([FT.new([], [Bool])], lambda stack: stack.append(cast(value)))


def cast(x):
    return TRUE if x else FALSE


def if_exec(stack):
    [cond, t, f] = stack[-3:]
    del stack[-3:]

    stack.append(t if cond == TRUE else f)


def if_func():
    gen = Generic("a")
    return Function([FT.new([Bool, gen, gen], [gen])], if_exec)


def eq_exec(stack: list):
    stack.append(cast(stack.pop() == stack.pop()))


def eq_func():
    gen = Generic("a")
    return Function([FT.new([gen, gen], [Bool])], eq_exec)


builtin_functions: dict[str, Callable[[], Function]] = {
    "?": if_func,
    "=": eq_func
}


def parse(token: str) -> Function:
    if token in ["TRUE", "FALSE"]:
        return bool_literal(token == "TRUE")
    if token in builtin_functions:
        return builtin_functions[token]()
