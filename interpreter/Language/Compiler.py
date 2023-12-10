from tree_sitter import Tree, Node

from Language.Typechecker import TypeChecker
from functions.Function import Function
from functions.Module import Module
from modules.Boolean import bool_literal
from modules.Numbers import number_literal
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

    @staticmethod
    def create_quote(context: Module.context, func: Function):
        return Function.new([FT.new([], [func.type])], lambda stack: stack.append(func.value))

    def create_binding(self, context: Module.context, name):
        t = GenericFunction.new()

        context[name] = lambda: Function(BoundType.new(name), lambda stack: self.runtime_context[name](stack))
        return Function(FunctionType(
            [BindingType(
                StackType.new([t]), StackType.new([]), name, t
            )]),
            (lambda stack: self.runtime_context.setdefault(name, stack.pop()))
        )

    def compile_node(self, context: Module.context, node: Node) -> Function | None:
        match node.type:
            case "identifier":
                return self.resolve_function_name(context, node.text.decode())
            case "function_binding":
                return self.create_binding(context, node.child_by_field_name("identifier").text.decode())
            case "quote":
                expression = node.child_by_field_name("expression")
                if expression is None:
                    return self.create_quote(context, self.resolve_function_name(context, "id"))
                return self.create_quote(context, self.compile_node(context, node.child_by_field_name("expression")))
            case "number":
                return number_literal(node.text)
            case "true":
                return bool_literal(True)
            case "false":
                return bool_literal(False)
            case "comment":
                return
            case _:
                raise CompilerException(f"Unhandled case {node}")

    def compile_program(self, context: Module.context, tree: Tree):
        root = tree.root_node
        if root.type != "program":
            raise CompilerException("Can only compile a program")
        for node in root.children:
            func = self.compile_node(context, node)
            if func is None:
                continue
            self.typechecker.check(func.type)
            yield func.value

    @property
    def stack(self):
        return self.typechecker.stack
