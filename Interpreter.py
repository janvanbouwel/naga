from collections.abc import Iterable

from Value import Value
from functions.Function import Function


def interpret(program: Iterable):
    stack = []

    for func in program:
        func(stack)

    return stack
