from .FunctionType import FunctionType
from .Generics import GenStack
from .StackType import StackType
from .Types import Type


class TypecheckException(SystemExit):
    pass


def typecheck(stack_func: FunctionType, func_type: FunctionType) -> list[FunctionType]:
    generics: dict[Type, Type] = {}

    stack_in = stack_func.in_type
    stack = stack_func.out_type

    in_type = func_type.in_type

    ret: list[FunctionType] = []

    while not in_type.empty:
        if in_type in generics:
            if in_type == generics[in_type]:
                stack_in = in_type.concat(stack_in)
                break
            in_type = generics[in_type]
            continue

        in_type, expected = in_type.pop()

        if stack.empty:
            stack = stack.prepend(expected)
            stack_in = stack_in.prepend(expected)

        if isinstance(stack, GenStack):
            ret.append(FunctionType(stack_in, stack))
            stack_in = in_type
            stack = StackType.new([])
            break

        stack, stack_top = stack.pop()

        res, generics = expected.match(stack_top, generics)
        if not res:
            raise TypecheckException(f"Mismatched types: expected: {expected}, was: {str(stack_top)}")

    out_type = func_type.out_type
    while not out_type.empty:
        if out_type in generics:
            if out_type == generics[out_type]:
                stack = out_type.concat(stack)
                break
            out_type = generics[out_type]
            continue

        out_type, out = out_type.pop()

        stack = stack.append(generics[out] if out in generics else out)

    return ret + [FunctionType(stack_in, stack)]
