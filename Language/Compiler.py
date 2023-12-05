from collections.abc import Iterable

from Language.Typechecker import TypeChecker
from functions.Binding import Binding
from functions.Function import Function
from functions.FunctionName import FunctionName
from functions.Module import Module
from functions.Quote import Quote
from functions.Symbol import Symbol
from type.BindingType import BindingType, BoundType
from type.FunctionType import FunctionType
from type.Generics import GenericFunction
from type.InstructionType import FT
from type.StackType import StackType


class CompilerException(Exception):
    pass


class Compiler:
    typechecker: TypeChecker
    runtime_context: dict

    def __init__(self):
        self.typechecker = TypeChecker()
        self.runtime_context = {}

    @staticmethod
    def resolve_function_name(context: Module.context, name: str) -> Function:
        if name not in context:
            raise CompilerException(f"Unknown function {name}")
        return context[name]()

    def resolve_quote(self, context: Module.context, symbol: Symbol) -> Function:
        func: Function
        match symbol:
            case Function(_, _):
                # noinspection PyTypeChecker
                func = symbol
            case FunctionName(name):
                func = self.resolve_function_name(context, name)
            case Quote(inner):
                func = self.resolve_quote(context, inner)
            case _:
                raise CompilerException(f"Symbol {symbol} can't be quoted.")
        return Function.new([FT.new([], [func.type])], lambda stack: stack.append(func.value))

    def create_binding(self, context: Module.context, name):
        t = GenericFunction.new("a")

        context[name] = lambda: Function(BoundType.new(name), lambda stack: self.runtime_context[name](stack))
        return Function(FunctionType(
            [BindingType(
                StackType.new([t]), StackType.new([]), name, t
            )]),
            (lambda stack: self.runtime_context.setdefault(name, stack.pop()))

        )

    def compile(self, context: Module.context, program: Iterable[Symbol]) -> list:
        for symbol in program:
            while True:
                match symbol:
                    case Function(t, value):
                        self.typechecker.check(t)
                        yield value
                        break
                    case FunctionName(name):
                        symbol = self.resolve_function_name(context, name)
                    case Quote(inner):
                        symbol = self.resolve_quote(context, inner)
                    case Binding(name):
                        print(name)
                        symbol = self.create_binding(context, name)
                    case _:
                        raise CompilerException(f"Unhandled case {symbol}")

    @property
    def stack(self):
        return self.typechecker.stack
