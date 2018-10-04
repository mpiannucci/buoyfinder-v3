
class MetaType:
    def __init__(self, rust_type, swift_type, kotlin_type, jni_type, jni_sig, cpp_type, opaque=False, params=None):
        self.rust_type = rust_type
        self.swift_type = swift_type
        self.kotlin_type = kotlin_type
        self.jni_type = jni_type
        self.jni_sig = jni_sig
        self.cpp_type = cpp_type
        self.opaque = opaque
        self.params = params

meta_type_map = {
    "i8": MetaType("i8", "Int8", "Byte", "jbyte", "B", "int8_t"),
    "i16": MetaType("i16", "Int16", "Short", "jshort", "S", "int16_t"),
    "i32": MetaType("i32", "Int32", "Int", "jint", "I", "int32_t"),
    "i64": MetaType("i64", "Int64", "Long", "jlong", "J", "int64_t"),
    "f32": MetaType("f32", "Float32", "Float", "jfloat", "F", "float"),
    "f64": MetaType("f64", "Double", "Double", "jdouble", "D", "double"),
    "bool": MetaType("bool", "Bool", "Boolean", "jboolean", "Z", "bool"),
}
