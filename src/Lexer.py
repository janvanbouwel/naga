from collections.abc import Iterable

from src.functions import Numbers, Builtins, Boolean


class ParseException(SystemExit):
    pass


parsers = [Numbers.parse, Builtins.parse, Boolean.parse]


def lex(tokens: Iterable[str]):
    for token in tokens:
        for parser in parsers:
            if (val := parser(token)) is not None:
                yield val
                break
        else:
            raise ParseException(f"Failed to parse token: {token}")
