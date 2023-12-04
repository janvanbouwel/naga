from __future__ import annotations

from abc import abstractmethod, ABC
from dataclasses import dataclass


@dataclass(frozen=True, eq=False)
class Type(ABC):
    @abstractmethod
    def match(self, other: Type, binding: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        pass


@dataclass(frozen=True)
class BaseType(Type):
    name: str

    def __repr__(self):
        return self.name

    def match(self, other: Type, binding: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        return self == other, binding
