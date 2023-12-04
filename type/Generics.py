from __future__ import annotations

from dataclasses import dataclass

from type.StackType import StackType
from type.Types import Type


@dataclass(eq=False, frozen=True)
class Generic(Type):
    name: str

    def __repr__(self):
        return f"Gen({self.name})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self

    def match(self, other: Type, binding: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if self in binding:
            return binding[self].match(other, binding)

        return True, binding | {self: other}


@dataclass(eq=False, frozen=True)
class GenStack(StackType):
    name: str

    def __len__(self):
        # raise Exception("Generic stack has no length")
        return 0

    def __copy__(self):
        return self

    def __deepcopy__(
            self,
    ):
        return self

    def __eq__(self, other):
        return True if self is other else NotImplemented

    def __hash__(self):
        return id(self)

    def present(self) -> str:
        return f"*{self.name}"

    def match(
            self, other: Type, binding: dict[Type, Type]
    ) -> tuple[bool, dict[Type, Type]]:
        if self in binding:
            return binding[self].match(other, binding)
        if not isinstance(other, StackType):
            return False, binding

        return True, binding | {self: other}
