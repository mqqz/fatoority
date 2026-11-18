import { readFileSync, writeFileSync } from 'node:fs';
import { InvoiceSigner } from 'fatoority';

const signer = InvoiceSigner.fromPem(
  readFileSync('certificate.pem', 'utf8'), readFileSync('private-key.pem', 'utf8'),
);
writeFileSync('signed.xml', signer.signXml(readFileSync('invoice.xml', 'utf8')));
console.log('Saved signed.xml');
