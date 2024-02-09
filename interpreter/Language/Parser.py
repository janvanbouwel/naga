from tree_sitter import Language, Parser, Tree


class ParseException(SystemExit):
    pass


def traverse_tree(tree: Tree):
    cursor = tree.walk()

    reached_root = False
    while not reached_root:
        yield cursor.node

        if cursor.goto_first_child():
            continue

        if cursor.goto_next_sibling():
            continue

        retracing = True
        while retracing:
            if not cursor.goto_parent():
                retracing = False
                reached_root = True

            if cursor.goto_next_sibling():
                retracing = False


def parse(content) -> Tree:
    Language.build_library("./build/lang.so", ["/home/jan/git/lang/tree-sitter-lang"])
    LANG = Language("build/lang.so", "lang")
    parser = Parser()

    parser.set_language(LANG)

    tree = parser.parse(content)

    return tree
