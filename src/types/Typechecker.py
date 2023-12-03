from collections.abc import Iterable

from src.types.Types import Type, FunctionType


class TypecheckException(SystemExit):
    pass


def typecheck(program: Iterable[FunctionType]):
    stack: list[Type] = []
    for func in program:
        if (count := len(func.in_type)) != 0:
            if (stack_top := stack[-count:]) != func.in_type:
                raise TypecheckException(f"Typechecking failed: top of stack: {stack_top}, func_in: {func.in_type}")
            del stack[-count:]
        stack += func.out_type
    return True
