from collections.abc import Iterable

from src.functions.Function import Function
from src.Value import Value


def interpret(program: Iterable[Function]):
    stack: list[Value] = []

    for func in program:
        args = []
        if func.argc > 0:
            args = stack[-func.argc:]
            del stack[-func.argc:]
        stack += func.value(args)

    return stack
