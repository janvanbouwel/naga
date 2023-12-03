from dataclasses import dataclass

from functions.Function import Function
from type.Types import Type


@dataclass(frozen=True)
class ArrayType(Type):
    type: list[Type]


def parse(token: str) -> Function:
    return
