from dataclasses import dataclass

from type.StackType import StackType
from type.Types import Type


@dataclass(frozen=True)
class FunctionType(Type):
    in_type: StackType
    out_type: StackType

    def __repr__(self):
        return f"F({str(self.in_type)}->{str(self.out_type)})"

    def match(self, other: Type, binding: dict[Type, Type]) -> tuple[bool, dict[Type, Type]]:
        if not isinstance(other, FunctionType):
            return False, binding

        res, binding = self.in_type.match(other.in_type, binding)
        if not res:
            return False, binding

        return self.out_type.match(other.out_type, binding)

    @staticmethod
    def new(in_type: list[Type], out_type: list[Type]):
        return FunctionType(StackType.new(in_type), StackType.new(out_type))


FT = FunctionType
