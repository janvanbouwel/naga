from abc import ABC, abstractmethod
from collections.abc import Callable

from functions.Function import Function


class Module(ABC):
    context = dict[str, Callable[[], Function]]

    @staticmethod
    @abstractmethod
    def built_in() -> context:
        pass
