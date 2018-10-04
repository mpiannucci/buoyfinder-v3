import ast
import meta


class IDLParser:

    def __init__(self, filenames):
        self.filenames = filenames
        self.idl_files = []
    
    def parse(self):
        for filename in self.filenames:
            with open(filename, 'r') as f:
                idl_input = f.read()
                idl_file = ast.IDLFile(filename, [])

                # Each type declaration has a body surrounded with braces, so we can use that to parse it quickly
                chunks = idl_input.split('}')
                for chunk in chunks:
                    breakdown = chunk.split('{')
                    decl = breakdown[0]
                    body = breakdown[1]

                    if 'record' in decl:
                        pass
                    elif 'interface' in decl:
                        pass
                    elif 'enum' in decl:
                        pass

                self.idl_files.append(idl_file)
