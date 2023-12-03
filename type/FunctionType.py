from dataclasses import dataclass

from type.Types import Type


@dataclass(frozen=True)
class FunctionType(Type):
    in_type: list[Type]
    out_type: list[Type]

    @property
    def argc(self):
        return len(self.in_type)

    def __repr__(self):
        return f"{repr(self.in_type)}->{repr(self.out_type)}"