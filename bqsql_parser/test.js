const rust = import('./pkg/bqsql_parser.js');

rust
    .then(m => {

        const p = m.parse('--my comments\nSELECT 2+2');

        console.info(JSON.stringify(p));

        const p2 = m.suggest('SELECT *, \nFROM dataset_id.table_id', 0, 10);
        console.info(JSON.stringify(p2));


    })
    .catch(console.error);