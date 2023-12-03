from collections.abc import Callable
from dataclasses import dataclass

from Value import Value
from type.FunctionType import FunctionType


@dataclass(frozen=True)
class Function(Value):
    type: FunctionType

    value: Callable[[list[Value]], list[Value]]

    @property
    def argc(self):
        return self.type.argc

    def __repr__(self):
        return f"{repr(self.type)}"
