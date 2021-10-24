#!/usr/bin/python

import io
import sys
import requests
import zipfile
import os.path
import textwrap
from hashlib import sha256


HERE = os.path.abspath(os.path.dirname(__file__))
BLVER = "1.0"
BLLINK = """https://www.cesys.com/fileadmin/user_upload/service/FPGA/""" \
         """fpga%%20boards%%20%%26%%20modules/BeastLink/""" \
         """beastlink-%s-windows-free.zip""" % BLVER

# Some sanity checks
BLLENGTH = 21136241
BLSHA256 = 'a1029b4adbbe5585d3fb762b89ea85efbe33bf8c5e526a52b1d2f991465b5c23'

DLLS = [
    'beastlink-{0}-windows-free/runtime/beastlink-{0}-x86.dll'.format(BLVER),
    'beastlink-{0}-windows-free/api/c++/lib/beastlink-{0}-x86.lib'.format(BLVER),
    'beastlink-{0}-windows-free/runtime/beastlink-{0}-x86_64.dll'.format(BLVER),
    'beastlink-{0}-windows-free/api/c++/lib/beastlink-{0}-x86_64.lib'.format(BLVER)
]


def main():
    print('Downloading beastlink zip... ', flush=True, end='')

    head = requests.head(BLLINK)
    length = int(head.headers['Content-Length'])
    if length != BLLENGTH:
        print('Incorrent content length: %d != %d' % (length, BLLENGTH), \
            file=sys.stderr)
        return 1

    with requests.get(BLLINK, stream=True) as req:
        try:
            req.raise_for_status()
        except requests.exceptions.HTTPError as err:
            print('HTTP Error:', err, file=sys.stderr)
            return 1

        rawbytes = req.content

        print('Done !')

        h = sha256(rawbytes).hexdigest()
        if h != BLSHA256:
            print('Invalid archive checksum:', h, file=sys.stderr)
            return 1
        else:
            print('Checksum validated !')

        with zipfile.ZipFile(io.BytesIO(rawbytes)) as zf:
            print('Extracting dlls... ', flush=True, end='')
            for dll in DLLS:
                data = zf.read(dll)
                outname = os.path.join(HERE, os.path.basename(dll))
                with open(os.path.join(outname), 'wb') as outfile:
                    outfile.write(data)
            print('Done !')

    return 0


def usage(prgname):
    print('\n%s [-f]: Downloads beastlink dlls' % \
        os.path.basename(sys.argv[0]))
    print('\n----')
    print(textwrap.fill('This script will download the necessary ' \
        'beastlink dlls for ArC2 development. ' \
        'By default this will only be done if you are on Windows. ' \
        'On Linux use appropriate distribution packages. ' \
        'If you still want to go ahead regardless of your platform ' \
        'add the `--force` or `-f` argument.'))
    print('----\n')
    sys.exit(0)


if __name__ == "__main__":


    if any(arg in sys.argv[1:] for arg in ['-h', '--help']):
        usage(sys.argv[0])

    force = any(arg in sys.argv[1:] for arg in ['-f', '--force'])

    if sys.platform != 'win32' and not force:
        usage(sys.argv[0])

    sys.exit(main())
