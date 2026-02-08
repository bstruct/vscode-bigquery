import path = require("path");
import { getExtensionUri } from "./extension";
import { IconPath, Uri } from "vscode";

export class BigqueryIcons {

    public bigquery: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'bigquery.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'bigquery.svg'))
    };

    public datasetLink: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'dataset-link.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'dataset-link.svg'))
    };

    public dataset: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'dataset.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'dataset.svg'))
    };

    public group: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'group.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'group.svg'))
    };

    public model: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'model.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'model.svg'))
    };

    public person: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'person.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'person.svg'))
    };

    public routine: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'routine.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'routine.svg'))
    };

    public tablePartitioned: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'table-partitioned.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'table-partitioned.svg'))
    };

    public tableView: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'table-view.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'table-view.svg'))
    };

    public table: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'table.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'table.svg'))
    };

    public pinned: IconPath = {
        light: Uri.file(path.join(getExtensionUri().path, 'resources', 'light', 'pinned.svg')),
        dark: Uri.file(path.join(getExtensionUri().path, 'resources', 'dark', 'pinned.svg'))
    };

}