
class Ident:
    def __init__(self, name):
        self.name = name


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


class Record:
    def __init__(self, ident, fields, constants, ext_langs):
        self.ident = ident
        self.fields = fields
        self.constants = constants
        self.ext_langs = ext_langs


class Method:
    def __init__(self, ident, params, static, const, ret_typ):
        self.ident = ident
        self.params = params
        self.static = static
        self.const = const
        self.ret_type = ret_typ


class Interface:
    def __init__(self, ident, methods, gets, sets, consts, ext_langs):
        self.ident = ident
        self.methods = methods
        self.gets = gets
        self.sets = sets
        self.consts = consts
        self.ext_langs = ext_langs


class IDLFile:
    def __init__(self, filename, user_types):
        self.filename = filename
        self.user_types = user_types