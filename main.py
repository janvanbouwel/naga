from Interpreter import interpret
from Parser import parse
from Tokenizer import tokenize
from type.Typechecker import typecheck

with open("test.lang") as f:
    tokens = [t for t in tokenize(f)]
    for token in tokens:
        print(token, end=" ")
    print()

    program = [i for i in parse(tokens)]
    print(f"program: {program}")

    typecheck(map(lambda x: x.type, program))

    print(interpret(program))
