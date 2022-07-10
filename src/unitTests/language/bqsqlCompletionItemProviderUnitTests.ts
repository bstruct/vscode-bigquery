// import * as vscode from 'vscode';
import { CancellationToken, CompletionContext, CompletionItem, CompletionList, Position, TextDocument } from "vscode";
import assert = require("assert");
import { BqsqlCompletionItemProvider } from '../../language/bqsqlCompletionItemProvider';

const bqsqlCompletionItemProvider = new BqsqlCompletionItemProvider();

describe('Array', function () {
  it('should return -1 when the value is not present', function () {

    const document = {} as TextDocument;
    const position = new Position(1, 2);
    const cancellationToken = {} as CancellationToken;
    const completionContext = {} as CompletionContext;

    const result = bqsqlCompletionItemProvider.provideCompletionItems(
      document, position, cancellationToken, completionContext
    ) as CompletionList<CompletionItem>;

    assert.strictEqual(result.items.length, 3);

  });
});