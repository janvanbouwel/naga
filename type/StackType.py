from __future__ import annotations
from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Sized, Iterable, Generator, Iterator
from dataclasses import dataclass

from .Types import Type


class StackException(Exception):
    pass


@dataclass(frozen=True)
class StackType(Type, Sized, ABC, Iterable):
    @staticmethod
    def new(types: list[Type]) -> StackType:
        stack = EmptyStack()
        for t in types:
            stack = stack.append(t)
        return stack

    def append(self, t: Type) -> StackType:
        return ConsStack(t, self)

    def pop(self):
        raise StackException("Can't pop empty stack")

    def __iter__(self) -> Iterator[StackType]:
        def gen() -> Generator[StackType, None, None]:
            current = self
            while True:
                match current:
                    case ConsStack(t, prev):
                        yield current
                        current = prev
                    case EmptyStack():
                        return
                    case _:
                        yield current
                        return

        return iter(gen())

    def __str__(self):
        return f"[{', '.join(s.present() for s in reversed(list(iter(self))))}]"

    @abstractmethod
    def present(self) -> str:
        pass


@dataclass(frozen=True)
class EmptyStack(StackType):
    def present(self) -> str:
        return ""

    def __len__(self):
        return 0

    def match(
            self, other: Type, binding: dict[Type, Type]
    ) -> tuple[bool, dict[Type, Type]]:
        return (self == other), binding


@dataclass(frozen=True)
class ConsStack(StackType):
    def present(self) -> str:
        return str(self.type)

    type: Type
    prev: StackType

    def __len__(self):
        return 1 + len(self.prev)

    def pop(self) -> tuple[StackType, Type]:
        return self.prev, self.type

    def match(
            self, other: Type, binding: dict[Type, Type]
    ) -> tuple[bool, dict[Type, Type]]:
        if not isinstance(other, ConsStack):
            return False, binding

        res, binding = self.type.match(other.type, binding)
        if not res:
            return False, binding

        return self.prev.match(other.prev, binding)
