from dataclasses import dataclass


@dataclass(frozen=True)
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

    def __repr__(self):
        return f"{repr(self.in_type)}->{repr(self.out_type)}"


class BaseTypes:
    Bool = BaseType("Bool")
    Int = BaseType("Int")
