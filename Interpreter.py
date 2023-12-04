from collections.abc import Iterable

from Value import Value
from functions.Function import Function


def interpret(program: Iterable[Function]):
    stack: list[Value] = []

    for func in program:
        func.value(stack)

    return stack
