const fs = require('fs');
const filers = require('./data/production/companies.json');

const arrayified = Object.values(filers);

const stream = fs.createWriteStream('./db/seeders/0001-filer-v2.sql');

stream.once('open', function(fd) {
    stream.write('BEGIN;\n');
    stream.write(getEntries());
    stream.write('COMMIT;\n');
});

function getEntries() {
    let entriesString = '';
    for (let i = 0, l = arrayified.length; i < l; i++) {
        console.log('filer: ', i);
        const filer = arrayified[i];
        // insert statement
        entriesString += `INSERT INTO filer (cik, names) VALUES ('${filer.CIK}', ARRAY [`;
        // add names to array
        (filer.Names || []).forEach(name => {
            entriesString += `'${name}', `
        });
        // remove final ', ' from string
        entriesString = entriesString.slice(0, entriesString.length - 2);
        // close insert statement
        entriesString += `]);`
        // new line
        entriesString += '\n'
    }
    return entriesString;
}