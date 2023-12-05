from collections.abc import Iterable


def interpret(program: Iterable):
    stack = []

    for func in program:
        func(stack)

    return stack
