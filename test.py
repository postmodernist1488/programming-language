#!/usr/bin/env python3

import sys
import os
import subprocess
import re
from time import time

from typing import Tuple

def compilation_error(path, compiler_output):
    print(f"ERROR: failed to compile {path}: {compiler_output.decode()}")

def build():
    process = subprocess.run(['cargo', 'build'], stdout=sys.stdout, stderr=sys.stderr)
    process.check_returncode()

def run(path) -> Tuple[int, bytes, bytes]:
    compiler = subprocess.run(['target/debug/programming-language', path, '-o', 'out.a'], stderr=subprocess.PIPE)
    if compiler.returncode != 0:
        compilation_error(path, compiler.stderr)
        compiler.check_returncode()
    process = subprocess.run(['./out.a'], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return process.returncode, process.stdout, process.stderr

def update_test(path):
    try:
        code, stdout, stderr = run(path)
    except subprocess.CalledProcessError:
        return
    test_path = os.path.splitext(path)[0] + '.test'
    with open(test_path, 'w+') as f:
        f.write('[code] ' + str(code) + '\n')
        f.write('[stdout] ' + stdout.decode())
        f.write('[stderr] ' + stderr.decode())
        f.flush()
        f.seek(0)
        print(f'Generated {test_path}:')
        print(f.read())

def parse_test(path):
    try:
        with open(path) as f:
            s = f.read()
    except FileNotFoundError:
        print('ERROR: No test', path)
        exit(1)
    pattern = r'\[code\]\s*(\d+)\s*\[stdout\] (.*?)\[stderr\] (.*?)'
    matches = re.search(pattern, s, re.DOTALL)
    if not matches:
        print(f'ERROR: failed to parse {path}')
        exit(1)
    code = int(matches.group(1))
    stdout = bytes(matches.group(2), 'utf-8')
    stderr = bytes(matches.group(3), 'utf-8')
    return code, stdout, stderr

def run_tests(path):
    #TODO: multithreading

    start = time()

    files = list(filter(lambda x: os.path.splitext(x)[1] == '.prl', os.listdir(path)))
    all = len(files)
    failed_tests_acc = []

    print(f'Running {all} tests\n')
    for file in files:
        full_path = path + '/' + file
        print(full_path, "... ", end='')
        test_file_path = os.path.splitext(full_path)[0] + '.test'
        expected = parse_test(test_file_path)
        actual = run(path + '/' + file)
        if expected == actual:
            print('OK')
        else:
            print('FAILED')
            failed_tests_acc.append((full_path, expected, actual))

    failed = len(failed_tests_acc)
    successful = all - failed

    print(f'\ntest result: {successful} successful; {failed} failed; finished in {round(time() - start, 2)}s\n')

    if failed > 0:
        print('Failures:\n')
        for path, expected, actual in failed_tests_acc:
            print(f'---- {path} ----')
            print(f'expected: {expected}')
            print(f'actual: {actual}')

def main():
    if len(sys.argv) - 1 < 2:
        print('Error: not enough arguments')
        return

    subcommand = sys.argv[1]
    path = sys.argv[2]

    build()

    if subcommand == 'update':
        update_test(path)
        os.remove('out.a')
    elif subcommand == 'run':
        run_tests(path)
    else:
        print(f'Error: unknown subcommand `{subcommand}`')



if __name__ == '__main__':
    main()
