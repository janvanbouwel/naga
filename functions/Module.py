from abc import ABC, abstractmethod
from collections.abc import Callable


from functions.Function import Function
from functions.Symbol import Symbol


class Module(ABC):
    context = dict[str, Callable[[], Function]]

    @staticmethod
    @abstractmethod
    def parse(token: str) -> Symbol:
        pass

    @staticmethod
    @abstractmethod
    def built_in() -> context:
        pass


