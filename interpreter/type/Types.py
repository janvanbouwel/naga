from __future__ import annotations

from abc import abstractmethod, ABC
from dataclasses import dataclass

from type.GenericPrinter import GenericPrinter


@dataclass(frozen=True, eq=False)
class Type(ABC):
    @abstractmethod
    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        pass

    def apply(self):
        pass

    def replace(self, generics: dict[Type, Type]) -> Type:
        return self

    def __str__(self):
        return self.show(GenericPrinter())

    @abstractmethod
    def show(self, printer: GenericPrinter):
        pass


@dataclass(frozen=True)
class BaseType(Type):
    name: str

    def __repr__(self):
        return self.name

    def show(self, printer: GenericPrinter):
        return self.name

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        return self == other, generics
