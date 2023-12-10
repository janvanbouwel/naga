from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Iterable, Generator, Iterator
from dataclasses import dataclass

from .GenericPrinter import GenericPrinter
from .Types import Type


class StackException(Exception):
    pass


@dataclass(frozen=True)
class StackType(Type, ABC, Iterable):
    empty = False

    @staticmethod
    def new(types: list[Type]) -> StackType:
        stack = EmptyStack()
        for t in types:
            stack = stack.append(t)
        return stack

    def append(self, t: Type) -> StackType:
        return ConsStack(t, self)

    @abstractmethod
    def prepend(self, t: Type) -> StackType:
        pass

    @abstractmethod
    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> StackType:
        pass

    def pop(self) -> tuple[StackType, Type]:
        raise StackException("Can't pop empty stack")

    def __iter__(self) -> Iterator[StackType]:
        def gen() -> Generator[StackType, None, None]:
            current = self
            while True:
                match current:
                    case ConsStack(t, prev):
                        yield t
                        current = prev
                    case _:
                        return

        return iter(gen())

    def concat(self, other: StackType) -> StackType:
        if self.empty:
            return other
        stack = self
        for t in reversed(list(iter(other))):
            stack = stack.append(t)
        return stack


@dataclass(frozen=True)
class EmptyStack(StackType):
    def prepend(self, t: Type) -> StackType:
        return ConsStack(t, self)

    empty = True

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> StackType:
        return self

    def show(self, printer: GenericPrinter):
        return ""

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        return (self == other), generics


@dataclass(frozen=True)
class ConsStack(StackType):
    def prepend(self, t: Type) -> StackType:
        return ConsStack(self.type, self.prev.prepend(t))

    def show(self, printer: GenericPrinter):
        if self.prev.empty:
            return self.type.show(printer)
        return f"{self.prev.show(printer)}, {self.type.show(printer)}"

    type: Type
    prev: StackType

    def pop(self) -> tuple[StackType, Type]:
        return self.prev, self.type

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if not isinstance(other, ConsStack):
            return False, generics

        res, generics = self.type.match(other.type, generics)
        if not res:
            return False, generics

        return self.prev.match(other.prev, generics)

    def replace(self, generics: dict[Type, Type], context: dict[str, Type]) -> StackType:
        return ConsStack(self.type.replace(generics, context), self.prev.replace(generics, context))
