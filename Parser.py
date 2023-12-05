from collections.abc import Iterable

from functions import Numbers, Builtins, Boolean, Array
from functions.Builtins import id_func
from functions.Function import Function
from type.FunctionType import FT


class ParseException(SystemExit):
    pass


parsers = [Numbers.parse, Builtins.parse, Boolean.parse, Array.parse]


def quote(func: Function) -> Function:
    return Function([FT.new([], [t]) for t in func.type], lambda stack: stack.append(func.value))


def parse_one(token: str) -> Function:
    lst = list(parse([token]))
    if len(lst) != 1:
        print(lst)
        raise ParseException(f"Failed to parse: {token}")
    return lst[0]


def parse(tokens: Iterable[str]):
    for token in tokens:
        if token.startswith("'"):
            token = token[1:]
            func = parse_one(token) if len(token) > 0 else id_func
            yield quote(func)
            continue
        for parser in parsers:
            if (val := parser(token)) is not None:
                yield val
                break
        else:
            raise ParseException(f"Failed to parse token: {token}")
