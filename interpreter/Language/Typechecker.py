from type.BindingType import BindingType, BoundType
from type.FunctionType import FunctionType
from type.Generics import GenericStack
from type.InstructionType import InstructionType
from type.StackType import StackType
from type.Types import Type


class TypecheckException(SystemExit):
    pass


class TypeChecker:
    stack: FunctionType
    context: dict[str, Type]

    def __init__(self):
        self.stack = FunctionType([InstructionType.new([], [])])
        self.context = {}

    def check(self, t: FunctionType):
        if isinstance(t, BoundType):
            if t.name not in self.context:
                raise TypecheckException(f"{t.name} not in context")
            t = self.context[t.name]
        for func_type in t.functions:
            self.stack = FunctionType(self.stack.functions[:-1] + self.typecheck(self.stack.functions[-1], func_type))

    def typecheck(self, stack_func: InstructionType, func_type: InstructionType) -> list[InstructionType]:
        generics: dict[Type, Type] = {}

        stack_in = stack_func.in_type
        stack = stack_func.out_type

        in_type = func_type.in_type

        ret: list[InstructionType] = []

        while not in_type.empty:
            if isinstance(in_type, GenericStack):
                if in_type in generics:
                    if in_type == generics[in_type]:
                        stack_in = in_type.concat(stack_in)
                        break
                    in_type = generics[in_type]
                    continue
                else:
                    ret.append(InstructionType(stack_in, stack))
                    stack_in = StackType.new([])
                    stack = StackType.new([])
                    # raise TypecheckException("Unbound generic stack")

            if isinstance(stack, GenericStack):
                ret.append(InstructionType(stack_in, stack))
                stack_in = StackType.new([])
                stack = StackType.new([])

            if stack.empty:
                stack_in = in_type.concat(stack_in)
                break

            in_type, expected = in_type.pop()

            stack, stack_top = stack.pop()

            res, generics = expected.match(stack_top, generics)
            if not res:
                raise TypecheckException(f"Mismatched types: expected: {expected}, was: {str(stack_top)}")

        stack = stack.concat(func_type.out_type.replace(generics, self.context))

        if isinstance(func_type, BindingType):
            if func_type.name in self.context:
                raise TypecheckException(f"Can't bind name {func_type.name} multiple times")
            self.context[func_type.name] = FunctionType(
                [func_type.binds.replace(generics, self.context)])

        return ret + [InstructionType(stack_in, stack)]
