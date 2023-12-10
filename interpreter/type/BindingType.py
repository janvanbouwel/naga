from dataclasses import dataclass

from type.FunctionType import FunctionType
from type.InstructionType import InstructionType
from type.Types import Type


@dataclass(frozen=True)
class BindingType(InstructionType):
    name: str
    binds: Type


@dataclass(frozen=True)
class BoundType(FunctionType):
    name: str

    @staticmethod
    def new(name: str):
        return BoundType([], name)
