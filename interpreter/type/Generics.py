from __future__ import annotations

from dataclasses import dataclass

from .FunctionType import FunctionType
from .GenericPrinter import GenericPrinter
from .InstructionType import InstructionType
from .StackType import StackType, StackException
from .Types import Type


@dataclass(eq=False, frozen=True)
class Generic(Type):
    def show(self, printer: GenericPrinter):
        return f"Gen({printer.get_name(self)})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if self in generics:
            return generics[self].match(other, generics)

        return True, generics | {self: other}

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> Type:
        if self in generics:
            return generics[self]
        return self


@dataclass(eq=False, frozen=True)
class GenericFunction(InstructionType):

    @staticmethod
    def new():
        return GenericFunction(GenericStack(), GenericStack())

    def show(self, printer: GenericPrinter):
        return f"GF({self.in_type.show(printer)}->{self.out_type.show(printer)})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if isinstance(other, FunctionType):
            if len(other.functions) == 1:
                return self.match(other.functions[0], generics)
            return True, generics | {self: other}
        return super().match(other, generics)

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> Type:
        if self in generics:
            return generics[self]

        in_type = self.in_type.replace(generics, context)
        out_type = self.out_type.replace(generics, context)

        if isinstance(in_type, GenericStack) or isinstance(out_type, GenericStack):
            return GenericFunction(in_type, out_type)
        return InstructionType(in_type, out_type)


@dataclass(eq=False, frozen=True)
class GenericStack(StackType):
    def prepend(self, t: Type) -> StackType:
        raise StackException("Idk yet what to do here if it is necessary")

    def __copy__(self):
        return self

    def __deepcopy__(self, ):
        return self

    def __eq__(self, other):
        return True if self is other else NotImplemented

    def __hash__(self):
        return id(self)

    def show(self, printer: GenericPrinter) -> str:
        return f"*{printer.get_name(self)}"

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if self in generics:
            return generics[self].match(other, generics)
        if not isinstance(other, StackType):
            return False, generics

        return True, generics | {self: other}

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> Type:
        if self in generics:
            return generics[self].replace(generics, context)
        return self
