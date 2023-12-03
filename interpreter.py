import copy
import sys

from src.Unique import Unique


class Tokens:
    quote = Unique("qt")
    unquote = Unique("un")


def add_tokens(tokens: list, add: list):
    for token in reversed(add):
        tokens.append(token)


def assertion(x):
    if not x: raise Exception("Assertion failed")


funcs = {
    "[]": (0, 1, lambda: [[]]),
    "+": (2, 1, lambda x, y: [x + y]),
    "print": (1, 0, lambda x: [print(x), []][-1]),
    "dup": (1, 0, lambda x: [x, copy.deepcopy(x)]),
    "len": (1, 2, lambda x: [x, len(x)]),
    "drop": (1, 0, lambda _: []),
    "?": (3, 1, lambda cond, true, false: [true if cond else false]),
    # "map": (2, 1, lambda lst, func: [ sum([ interpret(copy.deepcopy(func), [token]) for token in lst] , [])]),
    "=": (2, 1, lambda x, y: [x == y]),
    "assert": (1, 0, lambda x: [assertion(x), []][-1]),
    "pop": (1, 2, lambda lst: [lst, lst.pop()]),
    "push": (2, 1, lambda lst, val: [lst.append(val), [lst]][-1]),
    "out": (1, 1, lambda lst: [lst[0]])
}


def interpret(instructions: list, stack: list, context):
    instructions.reverse()
    trace = []
    quote = 0

    while len(instructions) > 0:
        instr = instructions.pop()

        # print(stack, token, [x for x in reversed(tokens)], context)

        match instr:
            case [*a]:
                stack.append(instr)
            case Tokens.quote:
                if quote > 0: stack.append(Tokens.quote)
                quote += 1
            case Tokens.unquote:
                quote -= 1
                if quote > 0: stack.append(Tokens.unquote)
            case "(":
                add_tokens(instructions, ["[", Tokens.quote])
            case "[":
                add_tokens(instructions, ["[]", "\\"])
            case ")":
                add_tokens(instructions, [Tokens.unquote, "/"])
            case _:
                if quote > 0:
                    stack.append(instr)
                elif instr == "\\":
                    trace.append(stack)
                    stack = stack[-1]
                elif instr in ["/", "]"]:
                    stack = trace.pop()
                elif instr.startswith("$"):
                    instr = instr[1:]
                    assert len(instr) > 0
                    assert instr not in funcs
                    context[instr] = stack.pop()
                elif instr.startswith("'"):
                    instr = instr[1:]
                    if len(instr) == 0:
                        stack.append([])
                    else:
                        stack.append([instr])
                elif instr in context:
                    func = context[instr]
                    interpret(copy.deepcopy(func), stack, copy.deepcopy(context))
                elif instr == "!":
                    func = stack.pop()
                    interpret(copy.deepcopy(func), stack, copy.deepcopy(context))
                elif instr in funcs:
                    argc, _, action = funcs[instr]
                    args = stack[-argc:] if argc > 0 else []
                    res = action(*args)
                    if argc > 0: del stack[-argc:]

                    stack += res

                else:
                    stack.append(int(instr))

    return stack, context


name = sys.argv[1] if len(sys.argv) > 1 else "example.lang"

with open(name, "r") as f:
    content = [line[0:-1] for line in f.readlines()]

    context = {}
    for line in content:
        if len(line.strip()) == 0: continue

        # print(line)

        if line.startswith("#"): continue
        # print([key for key in context.keys()])
        tokens = line.split(" ")
        res, context = interpret(tokens, [], copy.deepcopy(context))
        if len(res) > 0: print(res)
