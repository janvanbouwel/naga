from dataclasses import dataclass


@dataclass(frozen=True, eq=False)
class Type:
    pass


@dataclass(frozen=True)
class BaseType(Type):
    name: str

    def __repr__(self):
        return self.name


@dataclass(frozen=True)
class FunctionType(Type):
    in_type: list[Type]
    out_type: list[Type]

    @property
    def argc(self):
        return len(self.in_type)

    def __repr__(self):
        return f"{repr(self.in_type)}->{repr(self.out_type)}"


@dataclass(eq=False, frozen=True)
class Generic(Type):
    name: str

    def __repr__(self):
        return f"Gen({self.name})"

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self
