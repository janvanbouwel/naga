from src.Interpreter import interpret
from src.Lexer import lex
from src.Tokenizer import tokenize
from src.types.Typechecker import typecheck

with open("test.lang") as f:
    tokens = [t for t in tokenize(f)]
    for token in tokens:
        print(token, end=" ")
    print()

    program = [i for i in lex(tokens)]
    print(program)

    typecheck(map(lambda x: x.type, program))

    print(interpret(program))
