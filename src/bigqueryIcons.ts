import path = require("path");
import { extensionUri } from "./extension";

export class BigqueryIcons {

    public bigquery: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'bigquery.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'bigquery.svg')
    };

    public datasetLink: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'dataset-link.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'dataset-link.svg')
    };

    public dataset: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'dataset.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'dataset.svg')
    };

    public group: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'group.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'group.svg')
    };

    public model: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'model.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'model.svg')
    };

    public person: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'person.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'person.svg')
    };

    public routine: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'routine.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'routine.svg')
    };

    public tablePartitioned: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table-partitioned.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table-partitioned.svg')
    };

    public tableView: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table-view.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table-view.svg')
    };

    public table: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'table.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'table.svg')
    };

    public pinned: { light: string; dark: string } = {
        light: path.join(extensionUri.path, 'resources', 'light', 'pinned.svg'),
        dark: path.join(extensionUri.path, 'resources', 'dark', 'pinned.svg')
    };

}