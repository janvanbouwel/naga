from __future__ import annotations

from dataclasses import dataclass

from .FunctionType import FunctionType
from .InstructionType import InstructionType
from .StackType import StackType, StackException
from .Types import Type


@dataclass(eq=False, frozen=True)
class Generic(Type):
    name: str

    def __repr__(self):
        return f"Gen({self.name})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if self in generics:
            return generics[self].match(other, generics)

        return True, generics | {self: other}

    def replace(self, generics: dict[Type, Type]) -> Type:
        if self in generics:
            return generics[self]
        return self


@dataclass(eq=False, frozen=True)
class GenericFunction(InstructionType):
    name: str

    @staticmethod
    def new(name: str):
        return GenericFunction(GenericStack(name), GenericStack(name), name)

    def __repr__(self):
        return f"GF({self.in_type}->{self.out_type})"

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

    def replace(self, generics: dict[Type, Type]) -> Type:
        if self in generics:
            return generics[self]

        in_type = self.in_type.replace(generics)
        out_type = self.out_type.replace(generics)

        if isinstance(in_type, GenericStack) or isinstance(out_type, GenericStack):
            return GenericFunction(in_type, out_type, self.name)
        return InstructionType(in_type, out_type)


@dataclass(eq=False, frozen=True)
class GenericStack(StackType):
    def prepend(self, t: Type) -> StackType:
        raise StackException("Idk yet what to do here if it is necessary")

    name: str

    def __copy__(self):
        return self

    def __deepcopy__(self, ):
        return self

    def __eq__(self, other):
        return True if self is other else NotImplemented

    def __hash__(self):
        return id(self)

    def show(self) -> str:
        return f"*{self.name}"

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if self in generics:
            return generics[self].match(other, generics)
        if not isinstance(other, StackType):
            return False, generics

        return True, generics | {self: other}

    def replace(self, generics: dict[Type, Type]) -> Type:
        if self in generics:
            return generics[self]
        return self
