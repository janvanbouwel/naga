from Language.Compiler import Compiler
from Language.Interpreter import interpret
from Language.Parser import parse
from Language.Tokenizer import tokenize
from functions.Module import Module
from modules.Base import Base
from modules.Boolean import Boolean
from modules.Numbers import Number

with open("test.lang") as f:
    tokens = [t for t in tokenize(f)]
    for token in tokens:
        print(token, end=" ")
    print()

    modules: list[Module] = [Base, Number, Boolean]

    ir = [i for i in parse(modules, tokens)]
    print(f"IR: {ir}")

    context = {}
    for module in modules:
        context |= module.built_in()

    compiler = Compiler()
    program = list(compiler.compile(context, ir))
    print(f"Resulting stack type: {compiler.stack}")
    if len(compiler.stack.functions) != 1 or not compiler.stack.functions[0].in_type.empty:
        raise Exception("Program expects non-empty stack")

    print(f"Output: {interpret(program)}")
