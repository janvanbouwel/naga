from __future__ import annotations

from dataclasses import dataclass

from type.GenericPrinter import GenericPrinter
from type.InstructionType import InstructionType
from type.Types import Type


@dataclass(frozen=True)
class FunctionType(Type):
    functions: list[InstructionType]

    def __repr__(self):
        return super().__repr__()

    def show(self, printer: GenericPrinter):
        return f"[{','.join(func.show(printer) for func in self.functions)}]"

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> FunctionType:
        return FunctionType([instr.replace(generics, context) for instr in self.functions])

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if len(self.functions) == 1:
            if isinstance(other, InstructionType):
                return self.functions[0].match(other, generics)

        if not isinstance(other, FunctionType) or len(self.functions) != len(other.functions):
            return False, generics

        for i1, i2 in zip(self.functions, other.functions):
            res, generics = i1.match(i2, generics)
            if not res:
                return False, generics

        return True, generics
