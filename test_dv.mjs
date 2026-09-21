import * as XLSX from './node_modules/xlsx/xlsx.mjs';

const wb = XLSX.utils.book_new();
const ws = XLSX.utils.aoa_to_sheet([['A','B','C']]);
ws['!dataValidation'] = [{ sqref: 'C2:C100', type: 'list', formula1: '"A,B,C"' }];
ws['!dataValidations'] = [{ sqref: 'C2:C100', type: 'list', formula1: '"A,B,C"' }];
XLSX.utils.book_append_sheet(wb, ws, 'Sheet1');

const buf = XLSX.write(wb, { type: 'buffer', bookType: 'xlsx' });
const read = XLSX.read(buf, { type: 'buffer', bookFiles: true });
const xml = read.files['xl/worksheets/sheet1.xml'].content;
console.log('Includes dataValidation:', xml.includes('dataValidation'));
console.log('XML snippet:', xml.slice(-500));
