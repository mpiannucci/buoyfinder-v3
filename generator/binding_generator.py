

class BindingGenerator:

    def __init__(self, namespace, parsed_asts):
        self.namespace = namespace
        self.asts = parsed_asts

    def generate_rust(self, out_dir):
        with open(out_dir + '/' + self.namespace + '.rs') as f:
            for item in self.asts:
                