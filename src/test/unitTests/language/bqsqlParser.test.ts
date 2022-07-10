import assert = require("assert");
import * as mocha from 'mocha';

import { BqsqlDocumentItemType } from "../../../language/bqsqlDocument";
import { BqsqlParser } from "../../../language/bqsqlParser";

mocha.suite('BqsqlParser - SELECT FROM table', function () {
    mocha.test('test1', function () {

        const result = BqsqlParser.parse(`-- test lasdjflasjdf
SELECT 
    pimExportDate, 
    Combi_number,
    
    -- Flavour_Copy
FROM \`damiao-project-1.PvhTest.PimExport\` pim
WHERE 
    pimExportDate = "2022-03-23"
    -- AND (
    --     Combi_number = '0000F3223E001'
    --     OR Combi_number = "0000F2934E101"
    -- )
LIMIT 101;`);


        assert.strictEqual(result.items.length, 2);
        assert.strictEqual(result.items[0].type, BqsqlDocumentItemType.comment);
        assert.strictEqual(result.items[1].type, BqsqlDocumentItemType.query);

    });
});