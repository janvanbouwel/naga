from collections.abc import Iterable, Generator


def tokenize(source: Iterable[str]) -> Generator[str, None, None]:
    for line in source:
        if (index := line.find("#")) >= 0:
            line = line[:index]

        line = line.strip()

        for token in line.split():
            token = token.strip()
            if len(token) == 0:
                continue

            yield token
