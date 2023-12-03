from dataclasses import dataclass
from typing import Any

from type.Types import Type


@dataclass(frozen=True)
class Value:
    type: Type
    value: Any

    def __repr__(self):
        return f"{self.type}: {self.value}"
