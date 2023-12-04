from collections.abc import Iterable

from functions.Function import Function
from type.FunctionType import FunctionType
from type.Typechecker import typecheck


class Compiler:
    stack: FunctionType

    def __init__(self):
        self.stack = FunctionType.new([], [])

    def compile(self, program: Iterable[Function]) -> list:
        for func in program:
            func_type = func.type
            self.stack = typecheck(self.stack, func_type)
            yield func.value
