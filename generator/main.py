from __future__ import print_function
import argparse
from pycparser import c_ast, parse_file


class Struct:
    def __init__(self, struct_node):
        self._node = struct_node
        self._func_nodes = []

    @property
    def name(self):
        return self._node.name

    def add_function(self, func_node):
        self._func_nodes.append(func_node)


structs = {}
functions = []


class Visitor(c_ast.NodeVisitor):

    def visit_Decl(self, node):
        if isinstance(node.type, c_ast.Struct):
            new_struct = Struct(node.type)
            if new_struct.name in structs:
                return
            structs[node.type.name] = node.type
        elif isinstance(node.type, c_ast.FuncDecl):
            print(node.name)


if __name__ == "__main__":
    argparser = argparse.ArgumentParser('Dump AST')
    argparser.add_argument('filename', help='name of file to parse')
    args = argparser.parse_args()

    ast = parse_file(args.filename, use_cpp=True,
                     cpp_path='gcc',
                     cpp_args=['-E',
                               r'-I../../pycparser/utils/fake_libc_include'])
    ast.show()
    v = Visitor()
    v.visit(ast)

    print(structs.keys())
