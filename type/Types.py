from dataclasses import dataclass


@dataclass(frozen=True, eq=False)
class Type:
    pass


@dataclass(frozen=True, eq=True)
class StackType(Type):
    pass


@dataclass(frozen=True)
class BaseType(Type):
    name: str

    def __repr__(self):
        return self.name


@dataclass(eq=False, frozen=True)
class Generic(Type):
    name: str

    def __repr__(self):
        return f"Gen({self.name})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self
