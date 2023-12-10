from dataclasses import dataclass

from type.FunctionType import FunctionType
from type.GenericPrinter import GenericPrinter
from type.InstructionType import InstructionType
from type.Types import Type


@dataclass(frozen=True)
class BindingType(InstructionType):
    name: str
    binds: Type


@dataclass(frozen=True)
class BoundType(FunctionType):
    name: str

    def show(self, printer: GenericPrinter):
        return f"Bound({self.name})"

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> FunctionType:
        if self.name in context:
            return context[self.name]
        return self

    @staticmethod
    def new(name: str):
        return BoundType([], name)
