from dataclasses import dataclass

from functions.Symbol import Symbol


@dataclass(frozen=True)
class Binding(Symbol):
    name: str

    def __repr__(self):
        return f"${self.name}"
