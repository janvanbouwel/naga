from type.FunctionType import FunctionType
from type.StackType import StackType
from type.Types import Type


class TypecheckException(SystemExit):
    pass


def typecheck(stack: FunctionType, func_type: FunctionType) -> FunctionType:
    stack_in = stack.in_type
    current_stack = stack.out_type

    in_type = func_type.in_type
    out_type = func_type.out_type

    generics: dict[Type, Type] = {}
    while not in_type.empty:
        if in_type in generics:
            in_type = generics[in_type]
            continue

        in_type, expected = in_type.pop()

        if current_stack.empty:
            current_stack = current_stack.prepend(expected)
            stack_in = stack_in.prepend(expected)

        current_stack, stack_top = current_stack.pop()

        res, generics = expected.match(stack_top, generics)
        if not res:
            raise TypecheckException(f"Mismatched types: expected: {expected}, was: {str(stack_top)}")

    while not out_type.empty:
        if out_type in generics:
            out_type = generics[out_type]
            continue

        out_type, out = out_type.pop()

        current_stack = current_stack.append(generics[out] if out in generics else out)

    return FunctionType(stack_in, current_stack)
