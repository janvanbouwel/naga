from __future__ import annotations
from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Iterable, Generator, Iterator
from dataclasses import dataclass

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

    def __str__(self):
        return f"[{self.show()}]"

    @abstractmethod
    def show(self) -> str:
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

    def show(self):
        return ""

    def match(self, other: Type, generics: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        return (self == other), generics


@dataclass(frozen=True)
class ConsStack(StackType):
    def prepend(self, t: Type) -> StackType:
        return ConsStack(self.type, self.prev.prepend(t))

    def show(self):
        if self.prev.empty:
            return str(self.type)
        return f"{self.prev.show()}, {self.type}"

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

    def replace(self, generics: dict[Type, Type]) -> Type:
        return ConsStack(self.type.replace(generics), self.prev.replace(generics))
