import { readFileSync } from 'node:fs';
import { Xml } from 'fatoority';

const xml = readFileSync('invoice.xml', 'utf8');
if (process.argv.includes('--zatca')) {
  const report = Xml.validateZatca(xml, JSON.stringify({
    previous_invoice_hash: readFileSync('previous-hash.txt', 'utf8').trim(),
  }));
  console.log(JSON.stringify(JSON.parse(report.toJson()), null, 2));
  process.exitCode = report.isValid() ? 0 : 1;
} else {
  Xml.validate(xml);
  console.log('XSD valid');
}
