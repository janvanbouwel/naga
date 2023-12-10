from dataclasses import dataclass

from type.GenericPrinter import GenericPrinter
from type.StackType import StackType
from type.Types import Type


@dataclass(frozen=True)
class InstructionType(Type):
    in_type: StackType
    out_type: StackType

    def __repr__(self):
        if self.in_type.empty:
            return f"C{self.out_type}"
        return f"I({str(self.in_type)}->{str(self.out_type)})"

    def show(self, printer: GenericPrinter):
        if self.in_type.empty:
            return f"C{self.out_type.show(printer)}"
        return f"I({self.in_type.show(printer)}->{self.out_type.show(printer)})"

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if not isinstance(other, InstructionType):
            return False, generics

        res, generics = self.in_type.match(other.in_type, generics)
        if not res:
            return False, generics

        return self.out_type.match(other.out_type, generics)

    def replace(self, generics: dict[Type, Type]) -> Type:
        return InstructionType(self.in_type.replace(generics), self.out_type.replace(generics))

    @staticmethod
    def new(in_type: list[Type], out_type: list[Type]):
        return InstructionType(StackType.new(in_type), StackType.new(out_type))


FT = InstructionType
