from mako.template import Template
from mako.lookup import TemplateLookup

cpp_template_lookup = TemplateLookup(directories=['./templates/cpp'], encoding_errors='replace')
rust_template_lookup = TemplateLookup(directories=['./templates/rust'], encoding_errors='replace')
swift_template_lookup = TemplateLookup(directories=['./templates/swift'], encoding_errors='replace')
kotlin_template_lookup = TemplateLookup(directories=['./templates/kotlin'], encoding_errors='replace')

class Ident:
    def __init__(self, name):
        self.name = name

    @property
    def snake_case(self):
        return self.name

    @property
    def camel_case(self):
        chunks = self.name.split('_')
        if len(chunks) < 2:
            return self.name
        return ''.join([chunks[0], ''.join([x.capitalize() for x in chunks[1:]])])
    
    @property
    def pascal_case(self):
        chunks = self.name.split('_')
        return ''.join([x.capitalize() for x in chunks])
    
    @property
    def screaming_snake_case(self):
        return self.name.upper()


class EnumValue:
    def __init__(self, ident, val):
        self.ident = ident
        self.val = val

class Constant: 
    def __init__(self, ident, typ, val):
        self.ident = ident
        self.typ = typ
        self.val = val


class Field: 
    def __init__(self, ident, typ):
        self.ident = ident
        self.typ = typ


class Enum:
    def __init__(self, ident, variants):
        self.ident = ident
        self.variants = variants

    def to_rust(self):
        return rust_template_lookup.get_template("enum.txt").render(enum=self)

    def to_swift(self):
        return swift_template_lookup.get_template("enum.txt").render(enum=self)
    
    def to_kotlin(self):
        return kotlin_template_lookup.get_template("enum.txt").render(enum=self)

    def to_cpp(self):
        return cpp_template_lookup.get_template("enum.txt").render(enum=self)


class Record:
    def __init__(self, ident, fields, constants, ext_langs):
        self.ident = ident
        self.fields = fields
        self.constants = constants
        self.ext_langs = ext_langs

    def to_rust(self):
        return ''

    def to_swift(self):
        return ''
    
    def to_kotlin(self):
        return ''

    def to_cpp(self):
        return ''


class Interface:
    def __init__(self, ident, methods, gets, sets, consts, ext_langs):
        self.ident = ident
        self.methods = methods
        self.gets = gets
        self.sets = sets
        self.consts = consts
        self.ext_langs = ext_langs

    def to_rust(self):
        return ''

    def to_swift(self):
        return ''
    
    def to_kotlin(self):
        return ''

    def to_cpp(self):
        return ''


class IDLFile:
    def __init__(self, filename, user_types):
        self.filename = filename
        self.user_types = user_types