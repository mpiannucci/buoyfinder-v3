import idl_ast
import idl_meta


class IDLParser:

    def __init__(self, filenames):
        self.filenames = filenames
        self.idl_files = []
        self.typemap = {}
    
    def parse(self):
        self.typemap = idl_meta.create_type_map()
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

                    if 'enum' in decl:
                        decl_split = decl.split('=')

                        enum_name = idl_ast.Ident(decl_split[0].strip())

                        enum_index = 0
                        variants = [x.strip() for x in body.strip().split(';') if x]
                        enum_variants = []
                        for variant in variants:
                            enum_variants.append(idl_ast.EnumValue(idl_ast.Ident(variant), enum_index))
                            enum_index += 1

                        new_enum = idl_ast.Enum(enum_name, enum_variants)
                        self.typemap[enum_name.name] = idl_meta.MetaTypeOpaque(enum_name.name, [])
                        idl_file.user_types.append(new_enum)

                    elif 'record' in decl:
                        decl_split = decl.split('=')

                        record_name = idl_ast.Ident(decl_split[0].strip())

                        lang_exts = self.parse_lang_exts(decl_split[1].strip())

                        fields = []
                        consts = []
                        members = [x for x in body.strip().split(';') if x]
                        for member in members:
                            if member.startswith('const'):
                                # TODO: Handle constant def
                                continue

                            new_field = self.parse_field(member)
                            fields.append(new_field)
                        
                        new_record = idl_ast.Record(record_name, fields, consts, lang_exts)
                        self.typemap[record_name.name] = idl_meta.MetaTypeOpaque(record_name.name, [])
                        idl_file.user_types.append(new_record)
                    elif 'interface' in decl:
                        decl_split = decl.split('=')

                        interface_name = idl_ast.Ident(decl_split[0].strip())
                        
                        # Its legal to return yourself... so handle that
                        self.typemap[interface_name.name] = idl_meta.MetaTypeOpaque(interface_name, [])

                        lang_exts = self.parse_lang_exts(decl_split[1].strip())

                        methods = []
                        getters = []
                        setters = []
                        consts = []
                        members = [x for x in body.strip().split(';') if x]
                        for member in members:
                            method_static = False
                            member = member.strip()
                            if member.startswith('const'):
                                # TODO: Handle constant def
                                continue
                            elif member.startswith('get'):
                                member = member.replace('get', '')
                                getters.append(self.parse_field(member))
                                continue
                            elif member.startswith('set'):
                                member = member.replace('set', '')
                                setters.append(self.parse_field(member))
                                continue
                            elif member.startswith('static'):
                                method_static = True
                                member = member.replace('static', '')

                            raw_method = member.strip().split('(')
                            method_name = idl_ast.Ident(raw_method[0].strip())
                            method_signature = [x for x in raw_method[1].split(')') if x]
                            method_fields = []
                            for field in method_signature[0].split(','):
                                method_fields.append(self.parse_field(field))
                            method_ret_type_name = None
                            method_ret_type_params = []
                            if len(method_signature) > 1:
                                method_ret_type_name = method_signature[1].replace(':', '').strip()
                                if '<' in method_ret_type_name:
                                    raw_params = method_ret_type_name.split('<')
                                    method_ret_type_name = raw_params[0].strip()
                                    params = raw_params[1][:-1].split(',')
                                    for param in params:
                                        method_ret_type_params.append(param.strip())

                            method_ret_type = None
                            if method_ret_type_name is not None:
                                method_ret_type = self.typemap[method_ret_type_name]
                                method_ret_type.generics = method_ret_type_params
                            new_method = idl_ast.Method(method_name, method_fields, method_static, method_ret_type)
                            methods.append(new_method)
                        
                        new_interface = idl_ast.Interface(interface_name, methods, getters, setters, consts, lang_exts)
                        idl_file.user_types.append(new_interface)

                self.idl_files.append(idl_file)

    def parse_lang_exts(self, declaration):
        lang_exts = []
        if '+' in declaration:
            langs = declaration.split('+')[1:]
            for lang in langs:
                lang_exts.append(lang.strip())
        return lang_exts

    def parse_field(self, field_data):
        raw_field = field_data.split(':')
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
        field_meta_type = self.typemap[field_type]
        field_meta_type.generics = [self.typemap[x] for x in field_params]
        return idl_ast.Field(field_ident, field_meta_type)
