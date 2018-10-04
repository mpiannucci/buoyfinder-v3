import argparse


if __name__ == '__main__':
    argparser = argparse.ArgumentParser('FFI Generator')
    argparser.add_argument('filename', help='name of idl input file')
    args = argparser.parse_args()
