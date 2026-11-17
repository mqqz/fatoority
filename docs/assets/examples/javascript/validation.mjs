import { readFileSync } from 'node:fs';
import { Xml } from 'fatoority';

const xml = readFileSync('invoice.xml', 'utf8');
if (process.argv.includes('--zatca')) {
  const report = Xml.validateZatca(xml, {
    previousInvoiceHash: readFileSync('previous-hash.txt', 'utf8').trim(),
  });
  console.log(JSON.stringify(report, null, 2));
  process.exitCode = report.isValid ? 0 : 1;
} else {
  Xml.validate(xml);
  console.log('XSD valid');
}
