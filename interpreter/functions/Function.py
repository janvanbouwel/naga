from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass

from type.FunctionType import FunctionType
from type.InstructionType import InstructionType


@dataclass(frozen=True)
class Function:
    type: FunctionType

    value: Callable[[list], None]

    @staticmethod
    def new(types: list[InstructionType], value: Function.value):
        return Function(FunctionType(types), value)
