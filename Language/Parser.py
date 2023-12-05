from collections.abc import Iterable, Generator

from functions.Binding import Binding
from functions.FunctionName import FunctionName
from functions.Module import Module
from functions.Quote import Quote
from functions.Symbol import Symbol
from modules.Base import id_func


class ParseException(SystemExit):
    pass


def quote(symbol: Symbol) -> Symbol:
    return Quote(symbol)


def parse_one(modules: list[Module], token: str) -> Symbol:
    lst = list(parse(modules, [token]))
    if len(lst) != 1:
        print(lst)
        raise ParseException(f"Failed to parse: {token}")
    return lst[0]


def parse(modules: list[Module], tokens: Iterable[str]) -> Generator[Symbol, None, None]:
    for token in tokens:
        if token.startswith("'"):
            token = token[1:]
            func = parse_one(modules, token) if len(token) > 0 else id_func
            yield quote(func)
            continue
        if token.startswith("$"):
            token = token[1:]
            if len(token) == 0:
                raise ParseException("Can't bind without name")
            symbol = parse_one(modules, token)
            if not isinstance(symbol, FunctionName):
                raise ParseException("Can only bind to name")
            yield Binding(symbol.name)
            continue
        for module in modules:
            if (val := module.parse(token)) is not None:
                yield val
                break
        else:
            yield FunctionName(token)
