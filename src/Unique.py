class Unique:
    def __init__(self, name=''):
        self.name = f"U({name})"

    def __repr__(self):
        return self.name

    def __copy__(self):
        return self

    def __deepcopy__(self, _):
        return self
