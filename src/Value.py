from dataclasses import dataclass
from typing import Any

from src.types.Types import Type


@dataclass(frozen=True)
class Value:
    type: Type
    value: Any
