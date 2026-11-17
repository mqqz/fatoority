#!/usr/bin/env python3
"""Smoke-test the packed npm artifact from an isolated installed consumer."""
from pathlib import Path
import argparse
import shutil
import json
import os
import subprocess
import tarfile
import tempfile
ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output-dir', type=Path, help='Keep the tested tarball for release')
args = parser.parse_args()
with tempfile.TemporaryDirectory(prefix='fatoora-js-package-') as temporary:
    directory = Path(temporary)
    env = {**os.environ, 'npm_config_cache': str(directory / 'cache')}
    packed = subprocess.check_output(['npm', 'pack', '--json', '--pack-destination', str(directory)], cwd=ROOT / 'bindings/js', text=True, env=env)
    packages = json.loads(packed)
    package = packages[0] if isinstance(packages, list) else next(iter(packages.values()))
    tarball = directory / package['filename']
    with tarfile.open(tarball) as archive:
        names = set(archive.getnames())
        for required in ('fatoora_ffi.wasm', 'index.mjs', 'index.d.mts', 'errors.mjs', 'errors.d.mts', 'patch.mjs', 'raw.mjs', 'generated/index.mjs', 'generated/index.d.mts', 'LICENSE-MIT', 'LICENSE-APACHE', 'THIRD_PARTY_NOTICES.md', 'licenses/zatca/LICENSE-LGPL-3.0.txt', 'licenses/zatca/LICENSE-GPL-3.0.txt', 'licenses/zatca/NOTICE.md'):
            assert f'package/{required}' in names, required
        manifest = json.load(archive.extractfile('package/package.json'))
        assert manifest['license'] == 'SEE LICENSE IN THIRD_PARTY_NOTICES.md'
        for export in manifest['exports'].values():
            assert export['default'] == export['import']
        notices = archive.extractfile('package/THIRD_PARTY_NOTICES.md').read()
        assert notices == (ROOT / 'THIRD_PARTY_NOTICES.md').read_bytes()
        for name in ('NOTICE.md', 'LICENSE-LGPL-3.0.txt', 'LICENSE-GPL-3.0.txt'):
            assert archive.extractfile(f'package/licenses/zatca/{name}').read() == (ROOT / 'fatoora-core/tests/fixtures/business-rules' / name).read_bytes()
        assert not any('/tests/' in name or '/node_modules/' in name for name in names)
    (directory / 'package.json').write_text('{"type":"module","private":true}')
    subprocess.run(['npm', 'install', '--offline', '--ignore-scripts', '--no-audit', '--no-fund', str(tarball)], cwd=directory, check=True, env=env)
    subprocess.run(['node', '--input-type=module', '-e', '''
import assert from 'node:assert/strict';
import { SigningKey, Config, ComplianceCsid, Environment, ZatcaClient, Address, InvalidInputError } from 'fatoority';
import * as raw from 'fatoority/raw';
assert.match(SigningKey.generate().toPem(), /BEGIN PRIVATE KEY/);
assert.equal(raw.Config, Config);
assert.throws(() => Address.create('SA', '\\0', '', '', '', null, null, null), InvalidInputError);
assert.throws(() => raw.Address.new_('SA', '\\0', '', '', '', null, null, null), InvalidInputError);
const client = new ZatcaClient(Config.create(Environment.NonProduction), {fetch: async () => new Response(JSON.stringify({requestID: 2, binarySecurityToken: 'new', secret: 'secret'}))});
const response = await client.requestProductionCsid(ComplianceCsid.create(Environment.NonProduction, '1', 'token', 'secret'));
assert.equal(response.binarySecurityToken(), 'new');
console.log('Installed npm package: WASM loading, key generation, typed errors and API response decoding passed');
'''], cwd=directory, check=True, env=env)
    if args.output_dir is not None:
        args.output_dir.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(tarball, args.output_dir / tarball.name)
