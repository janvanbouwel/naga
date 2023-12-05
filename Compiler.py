from collections.abc import Iterable

from functions.Function import Function
from type.FunctionType import FunctionType
from type.Typechecker import typecheck


class Compiler:
    stack: list[FunctionType]

    def __init__(self):
        self.stack = [FunctionType.new([], [])]

    def compile(self, program: Iterable[Function]) -> list:
        for func in program:
            for func_type in func.type:
                self.stack += typecheck(self.stack.pop(), func_type)
            yield func.value
