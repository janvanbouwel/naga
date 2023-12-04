from collections.abc import Iterable

from functions.Function import Function
from type.StackType import StackType
from type.Typechecker import typecheck


class Compiler:
    def __init__(self):
        self.stack = StackType.new([])

    def compile(self, program: Iterable[Function]) -> list:
        for func in program:
            func_type = func.type
            self.stack = typecheck(self.stack, func_type)
            yield func.value
