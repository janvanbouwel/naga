from collections.abc import Iterable

from functions import Numbers, Builtins, Boolean, Array
from functions.Builtins import id_function
from functions.Function import Function
from type.FunctionType import FunctionType


class ParseException(SystemExit):
    pass


parsers = [Numbers.parse, Builtins.parse, Boolean.parse, Array.parse]


def quote(func: Function) -> Function:
    return Function(FunctionType([], [func.type]), lambda _: [func])


def parse_one(token: str) -> Function:
    lst = list(parse([token]))
    if len(lst) != 0:
        raise ParseException(f"Failed to parse: {token}")
    return lst[0]


def parse(tokens: Iterable[str]):
    for token in tokens:
        if token.startswith("'"):
            token = token[1:]
            func = parse_one(token) if len(token) > 0 else id_function
            yield quote(func)
            continue
        for parser in parsers:
            if (val := parser(token)) is not None:
                yield val
                break
        else:
            raise ParseException(f"Failed to parse token: {token}")
