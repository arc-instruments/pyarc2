#!/usr/bin/env python

import re
import os.path
import requests
import tomli
import sys
import json
import subprocess
import shutil
import semver


def current_tag():
    try:
        out = subprocess.check_output([\
            'git', 'name-rev', '--name-only', '--no-undefined', '--tags', 'HEAD'],
            stderr=subprocess.DEVNULL)
        out = re.split('[\^\,\+]', out.decode().strip())
        semver.parse(out[0])
        return out[0]
    except (subprocess.CalledProcessError, ValueError):
        return None


def latest_tag():
    git = shutil.which('git')

    cmd = [git, 'tag', '--sort=taggerdate']

    lines = subprocess.check_output(cmd).decode().splitlines()

    try:
        return lines[-1]
    except IndexError:
        return None


def highest_semver_tag():
    git = shutil.which('git')

    cmd = [git, 'tag']

    lines = subprocess.check_output(cmd).decode().splitlines()

    return str(max(map(semver.VersionInfo.parse, lines)))


def docs_version():

    regexp = re.compile('^release\s?=\s?(.*)$')
    lines = open(os.path.join('docs', 'conf.py')).readlines()

    for line in lines:
        line = line.strip()
        if regexp.match(line):
            m = regexp.match(line)
            return m.group(1).replace("'", "")

    raise ValueError('Could not determine docs version')


def internal_version():

    cargo = tomli.loads(open('Cargo.toml').read())['package']['version']
    pyproject_tool = tomli.loads(open('pyproject.toml').read())['tool']['poetry']['version']
    pyproject = tomli.loads(open('pyproject.toml').read())['project']['version']
    docs = docs_version()

    versions = [cargo, pyproject, pyproject_tool, docs]

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


def pypi_versions():

    data = requests.get('https://pypi.org/pypi/pyarc2/json')

    # project not found, that's fine will be created now
    if data.status_code == 404:
        return []
    elif data.status_code != 200:
        raise Exception('Could not determine PyPI version')

    content = json.loads(data.content)
    versions = list(content['releases'].keys())

    return versions


if __name__ == "__main__":

    if sys.argv[1] == 'commitcheck':
        try:
            iver = internal_version()
            print('Found internal version:', iver)
        except ValueError as err:
            print('Repository versions are not consistent', file=sys.stderr)
            sys.exit(1)

        maxver = highest_semver_tag()

        if maxver is not None and semver.compare(maxver, iver) > 0:
            print('Current repository version is not higher than latest tag; '\
                'bump versions', file=sys.stderr)
            sys.exit(1)

    if sys.argv[1] == 'releasecheck':
        try:
            iver = internal_version()
            print('Found internal version:', iver)
        except ValueError as err:
            print('Repository versions are not consistent', file=sys.stderr)
            sys.exit(1)

        maxver = latest_tag()
        curtag = current_tag()

        if maxver is None:
            print('Cannot find latest tag', file=sys.stderr)
            sys.exit(1)

        if maxver is not None and semver.compare(maxver, iver) > 0:
            print('Current repository version is not higher than latest tag; '\
                'bump versions', file=sys.stderr)
            sys.exit(1)

        if maxver != curtag:
            print('Latest and highest tag are not the same', maxver, curtag, file=sys.stderr)
            sys.exit(1)

        try:
            pypivers = pypi_versions()
            print('Found all PyPI versions:', pypivers)
        except Exception as exc:
            print('A problem occurred when checking PyPI versions', exc, \
                file=sys.stderr)
            sys.exit(2)

        if iver in pypivers:
            print('An identical release exists on PyPI; bump versions '
                'before proceeding', file=sys.stderr)
            sys.exit(1)

    sys.exit(0)
