from Compiler import Compiler
from Interpreter import interpret
from Parser import parse
from Tokenizer import tokenize

with open("test.lang") as f:
    tokens = [t for t in tokenize(f)]
    for token in tokens:
        print(token, end=" ")
    print()

    ir = [i for i in parse(tokens)]
    print(f"IR: {ir}")

    compiler = Compiler()
    program = list(compiler.compile(ir))
    print(f"Resulting stack type: {compiler.stack}")

    print(interpret(program))
