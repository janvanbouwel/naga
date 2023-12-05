from dataclasses import dataclass

from functions.Symbol import Symbol


@dataclass(frozen=True)
class Quote(Symbol):
    symbol: Symbol

    def __repr__(self):
        return f"'{self.symbol}"
