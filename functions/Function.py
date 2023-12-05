from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass

from functions.Symbol import Symbol
from type.InstructionType import InstructionType
from type.FunctionType import FunctionType


@dataclass(frozen=True)
class Function(Symbol):
    type: FunctionType

    value: Callable[[list], None]

    def __repr__(self):
        return f"{self.type}"

    @staticmethod
    def new(types: list[InstructionType], value: Function.value):
        return Function(FunctionType(types), value)
