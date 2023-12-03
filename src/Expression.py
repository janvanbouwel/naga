from collections.abc import Callable
from dataclasses import dataclass

from src.types.Types import FunctionType
from src.Value import Value


@dataclass(frozen=True)
class Function(Value):
    type: FunctionType

    value: Callable[[list[Value]], list[Value]]

    def __repr__(self):
        return f"F({repr(self.type)})"
