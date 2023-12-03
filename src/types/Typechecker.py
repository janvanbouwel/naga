from collections.abc import Iterable

from src.types.Types import Type, FunctionType, Generic


class TypecheckException(SystemExit):
    pass


def typecheck(program: Iterable[FunctionType]):
    stack: list[Type] = []
    for func in program:
        if func.argc == 0:
            stack += func.out_type
            continue

        stack_top = stack[-func.argc:]
        if len(stack_top) != func.argc:
            raise TypecheckException(f"Typechecking failed: top of stack: {stack_top}, func_in: {func.in_type}")

        combined = zip(func.in_type, stack_top)
        generics: dict[Type, Type] = {}

        for expected, actual in zip(func.in_type, stack_top):
            if isinstance(expected, Generic):
                if expected in generics and generics[expected] != actual:
                    raise TypecheckException("mismatched types")
                generics[expected] = actual
            else:
                if expected != actual:
                    raise TypecheckException("mismatched types")

        del stack[-func.argc:]

        for out in func.out_type:
            stack.append(generics[out] if out in generics else out)

    return True
