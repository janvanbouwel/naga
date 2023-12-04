from type.FunctionType import FunctionType
from type.StackType import StackType, EmptyStack
from type.Types import Type


class TypecheckException(SystemExit):
    pass


def typecheck(stack: StackType, func_type: FunctionType) -> StackType:
    in_type = func_type.in_type
    out_type = func_type.out_type

    binding: dict[Type, Type] = {}
    while not isinstance(in_type, EmptyStack):
        if in_type in binding:
            in_type = binding[in_type]
            continue

        if len(stack) == 0:
            raise TypecheckException("Insufficient items on stack.")

        stack, stack_top = stack.pop()
        in_type, expected = in_type.pop()

        res, binding = expected.match(stack_top, binding)
        if not res:
            raise TypecheckException(f"Mismatched types: expected: {expected}, was: {str(stack_top)}")

    while not isinstance(out_type, EmptyStack):
        if out_type in binding:
            out_type = binding[out_type]
            continue

        out_type, out = out_type.pop()

        stack = stack.append(binding[out] if out in binding else out)

    return stack
