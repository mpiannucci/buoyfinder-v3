import idl_ast
import idl_meta


class IDLParser:

    def __init__(self, filenames):
        self.filenames = filenames
        self.idl_files = []
    
    def parse(self):
        typemap = idl_meta.create_type_map()
        for filename in self.filenames:
            with open(filename, 'r') as f:
                idl_input = f.read()
                idl_file = idl_ast.IDLFile(filename, [])

                # Each type declaration has a body surrounded with braces, so we can use that to parse it quickly
                chunks = [x for x in idl_input.split('}') if x]
                for chunk in chunks:
                    breakdown = chunk.split('{')
                    decl = breakdown[0]
                    body = breakdown[1]

                    if 'record' in decl:
                        decl_split = decl.split('=')

                        # Record Name
                        record_name = idl_ast.Ident(decl_split[0].strip())

                        # Language extensions
                        lang_exts = []
                        if '+' in decl_split[1]:
                            langs = decl_split[1].split('+')[1:]
                            for lang in langs:
                                lang_exts.append(lang.strip())

                        # Fields and constants
                        fields = []
                        consts = []
                        members = [x for x in body.strip().split(';') if x]
                        for field in members:
                            if field.startswith('const'):
                                # TODO: Handle constant def
                                continue

                            raw_field = field.split(':')
                            field_name = raw_field[0].strip()
                            field_type = raw_field[1].strip()
                            field_params = []
                            if '<' in field_type:
                                raw_params = field_type.split('<')
                                field_type = raw_params[0].strip()
                                params = raw_params[1][:-1].split(',')
                                for param in params:
                                    field_params.append(param.strip())

                            field_ident = idl_ast.Ident(field_name)
                            field_meta_type = typemap[field_type]
                            field_meta_type.generics = [typemap[x] for x in field_params]
                            fields.append(idl_ast.Field(field_ident, field_meta_type))
                        
                        new_record = idl_ast.Record(record_name, fields, consts, lang_exts)
                        typemap[record_name.name] = idl_meta.MetaTypeOpaque(record_name.name, [])
                        idl_file.user_types.append(new_record)
                    elif 'interface' in decl:
                        pass
                    elif 'enum' in decl:
                        pass

                self.idl_files.append(idl_file)
