#!/usr/bin/python

import os
import os.path
__HERE__ = os.path.dirname(__file__)
__ABOVE__ = os.path.dirname(__HERE__)
import sys
sys.path.append(os.path.dirname(__HERE__))

import argparse
import re
import time
import semver
import subprocess
import ftputil
import tomli
from pathlib import PurePosixPath


def join_paths(*args):
    return PurePosixPath(*args).as_posix()


def relpath(p, anchor):
    p = PurePosixPath(p)
    return p.relative_to(anchor).as_posix()


def basename(p):
    p = PurePosixPath(p)
    return p.name


def dirname(p):
    p = PurePosixPath(p)
    return p.parent.as_posix()


def find_git_version():
    try:
        out = subprocess.check_output([\
            'git', 'name-rev', '--name-only', '--no-undefined', '--tags', 'HEAD'])
        out = re.split('[\^\,\+]', out.decode().strip())
        semver.parse(out[0])
        return out[0]
    except (subprocess.CalledProcessError, ValueError):
        return 'latest'


def docs_version():

    regexp = re.compile('^release\s?=\s?(.*)$')
    lines = open(os.path.join(__HERE__, 'conf.py')).readlines()

    for line in lines:
        line = line.strip()
        if regexp.match(line):
            m = regexp.match(line)
            return m.group(1).replace("'", "")

    raise ValueError('Could not determine docs version')


def find_local_version():

    cargo = tomli.loads(open(os.path.join(__ABOVE__, 'Cargo.toml')).read())['package']['version']
    pyproject_tool = tomli.loads(open(os.path.join(__ABOVE__, 'pyproject.toml')).read())['tool']['poetry']['version']
    pyproject = tomli.loads(open(os.path.join(__ABOVE__, 'pyproject.toml')).read())['project']['version']

    versions = [cargo, pyproject_tool, pyproject, docs_version()]
    print(versions, file=sys.stderr)

    # Check if the same version is used throughout
    consistent = all(v == versions[0] for v in versions)
    if len(versions) < 4:
        raise ValueError('Not all of Cargo.toml, pyproject.toml or docs/conf.py '
            'define versions')
    elif not consistent:
        # complain if it doesn't
        raise ValueError('Cargo.toml, pyproject.toml and docs/conf.py '
            'have inconsistent versions')
    else:
        # return it otherwise
        return versions[0]


def main(project, host, username, password):
    gitv = find_git_version()
    if gitv != 'latest':
        try:
            docv = find_local_version()
        except ValueError:
            print('Local version inconsistency', file=sys.stderr)
            return 1

        if docv != gitv and gitv != 'latest':
            print('Git and software versions do not match; aborting', file=sys.stderr)
            return 1

        print('docv:', docv, 'gitv:', gitv)
    else:
        print('gitv:', gitv)

    version = gitv
    basedir = os.path.join(__HERE__, '_build', 'html')
    basedir_normalised = join_paths(__HERE__, '_build', 'html')

    print('Upload version target:', gitv)
    time.sleep(2)

    with ftputil.FTPHost(host, username, password) as ftp:
        dirs = []
        ftp.chdir(join_paths('documents', project))
        files = ftp.listdir(ftp.curdir)
        for f in files:
            if ftp.path.isfile(f) and f == version:
                print("%s exists and it's not a directory" % x, file=sys.stderr)
                return 1
            if ftp.path.isdir(f):
                dirs.append(f)

        if version in dirs:
            print('Version directory exists')
        else:
            ftp.mkdir(version)

        for (root, dirs, files) in os.walk(basedir):
            for f in files:
                source = os.path.join(root, f)
                source_normalised = join_paths(root, f)
                path = relpath(source_normalised, basedir_normalised)

                d = dirname(path)
                target = join_paths(version, d, basename(path))

                ftp.makedirs(join_paths(version, d), exist_ok=True)
                print('Uploading', relpath(source, basedir), 'to', target)
                ftp.upload(source, target)


if __name__ == "__main__":

    try:
        project = sys.argv[1]
    except IndexError:
        print('Usage: %s [project]' % sys.argv[0])
        sys.exit(1)

    try:
        host = os.environ['UPLOAD_HOST']
        user = os.environ['UPLOAD_USER']
        pw = os.environ['UPLOAD_PASSWORD']
    except KeyError as ke:
        print('Environment not setup properly:', ke, file=sys.stderr)
        sys.exit(1)

    sys.exit(main(project, host, user, pw))
