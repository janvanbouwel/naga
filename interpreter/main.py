from typing import BinaryIO

from Language.Compiler import Compiler
from Language.Interpreter import interpret
from Language.Parser import parse, traverse_tree
from functions.Module import Module
from modules.Base import Base
from modules.Boolean import Boolean
from modules.Numbers import Numbers


def read_callable(file: BinaryIO, offset: int):
    if file.tell() != offset:
        print("had to seek")
        file.seek(offset)
    return file.read(1)


with open("test.lang", "rb") as f:
    modules: list[Module] = [Base, Numbers, Boolean]

    content = f.read(-1)

    tree = parse(content)
    for t in traverse_tree(tree):
        if not t.is_named: continue
        # print(f"{t.type} {t.text} {t.child_count} {t.is_named}")

    context = {}
    for module in modules:
        context |= module.built_in()

    compiler = Compiler()
    program = list(compiler.compile_program(context, tree))
    print(f"Resulting stack type: {compiler.stack}")
    if len(compiler.stack.functions) != 1 or not compiler.stack.functions[0].in_type.empty:
        raise Exception("Program expects non-empty stack")

    print(f"Output: {interpret(program)}")
