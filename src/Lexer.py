from collections.abc import Iterable

from src.functions.Builtins import builtin_functions
from src.functions.Numbers import int_literal


class ParseException(SystemExit):
    pass


def lex(tokens: Iterable[str]):
    for token in tokens:
        try:
            if token in builtin_functions:
                yield builtin_functions[token]
            else:
                yield int_literal(int(token))
        except (Exception, ValueError):
            raise ParseException(f"Failed to parse token: {token}")
