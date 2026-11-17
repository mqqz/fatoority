import { readFileSync, writeFileSync } from 'node:fs';
import { CsrProperties, Environment, SigningKey } from 'fatoority';

const properties = CsrProperties.fromStr(readFileSync('csr.properties', 'utf8'));
const key = SigningKey.generate();
const csr = properties.build(key, Environment.NonProduction);
writeFileSync('csr.pem', csr.toPem());
writeFileSync('key.pem', key.toPem());
console.log('Saved csr.pem and key.pem');
