
class MetaTypePrimitive:
    def __init__(self, idl_type, rust_type, swift_type, kotlin_type, jni_type, jni_sig, cpp_type):
        self.idl_type = idl_type
        self.rust_type = rust_type
        self.swift_type = swift_type
        self.kotlin_type = kotlin_type
        self.jni_type = jni_type
        self.jni_sig = jni_sig
        self.cpp_type = cpp_type


class MetaTypeOpaque:
    def __init__(self, idl_type, generics):
        self.idl_type = idl_type
        self.generics = generics


def create_type_map():
    return {
        'i8': MetaTypePrimitive('i8', 'i8', 'Int8', 'Byte', 'jbyte', 'B', 'int8_t'),
        'i16': MetaTypePrimitive('i16', 'i16', 'Int16', 'Short', 'jshort', 'S', 'int16_t'),
        'i32': MetaTypePrimitive('i32', 'i32', 'Int32', 'Int', 'jint', 'I', 'int32_t'),
        'i64': MetaTypePrimitive('i64', 'i64', 'Int64', 'Long', 'jlong', 'J', 'int64_t'),
        'f32': MetaTypePrimitive('f32', 'f32', 'Float32', 'Float', 'jfloat', 'F', 'float'),
        'f64': MetaTypePrimitive('f64', 'f64', 'Double', 'Double', 'jdouble', 'D', 'double'),
        'bool': MetaTypePrimitive('bool', 'bool', 'Bool', 'Boolean', 'jboolean', 'Z', 'bool'),
        'string': MetaTypeOpaque('string', []),
        'optional': MetaTypeOpaque('optional', []),
        'list': MetaTypeOpaque('list', []),
        'set': MetaTypeOpaque('set', []),
        'map': MetaTypeOpaque('map', []),
    }
