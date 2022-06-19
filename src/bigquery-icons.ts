import path = require("path");
import { extensionUri } from "./extension";

export class BigqueryIcons {

    public Bigquery: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'bigquery.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'bigquery.svg')
    };

    public DatasetLink: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'dataset-link.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'dataset-link.svg')
    };

    public Dataset: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'dataset.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'dataset.svg')
    };

    public Group: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'group.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'group.svg')
    };

    public Model: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'model.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'model.svg')
    };

    public Person: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'person.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'person.svg')
    };

    public Routine: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'routine.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'routine.svg')
    };

    public TablePartitioned: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table-partitioned.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table-partitioned.svg')
    };

    public TableView: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table-view.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table-view.svg')
    };

    public Table: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table.svg')
    };

}