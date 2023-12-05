from dataclasses import dataclass

from functions.Symbol import Symbol


@dataclass(frozen=True)
class FunctionName(Symbol):
    name: str

    def __repr__(self):
        return f"FN({self.name})"
