// import * as vscode from 'vscode';
import * as assert from 'assert';
import { CancellationToken, CompletionContext, CompletionItem, CompletionList, Position, TextDocument } from 'vscode';
import { BqsqlCompletionItemProvider } from '../../../language/bqsqlCompletionItemProvider';

const bqsqlCompletionItemProvider = new BqsqlCompletionItemProvider();

suite('Array', function () {
  test('should return -1 when the value is not present', function () {

    const document = {} as TextDocument;
    const position = new Position(1, 2);
    const cancellationToken = {} as CancellationToken;
    const completionContext = {} as CompletionContext;

    const result = bqsqlCompletionItemProvider.provideCompletionItems(
      document, position, cancellationToken, completionContext
    ) as CompletionList<CompletionItem>;

    assert.strictEqual(result.items.length, 1);

  });
});