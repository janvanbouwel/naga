class GenericPrinter:
    generics: dict[object, str]

    def __init__(self):
        self.generics = {}

    def get_name(self, obj: object):
        if obj not in self.generics:
            self.generics[obj] = chr(65 + len(self.generics))
        return self.generics[obj]
