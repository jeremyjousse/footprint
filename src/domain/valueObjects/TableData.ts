import { toSnakeCase } from "$lib/helpers";

export enum DataTableColumnFilterType {
  String,
  Boolean,
  List,
}

export type TableColumnFilter = {
  type: DataTableColumnFilterType;
  values?: string[];
};

export type TableColumn = {
  name: string;
  field: string;
  displayHelper?: (value: any) => string;
  filter?: TableColumnFilter;
};

enum TableSortOrder {
  Asc = "Asc",
  Desc = "Desc",
}

type TableSortDto = {
  by: string;
  order: TableSortOrder;
};

class TableSort {
  constructor(by: string) {
    this.by = by;
    this.order = TableSortOrder.Asc;
  }

  by: string;
  order: TableSortOrder;

  toBackend = (): TableSortDto => {
    return {
      by: toSnakeCase(this.by),
      order: this.order,
    };
  };
}

export { TableSort, TableSortOrder };

export type TableRow = {
  [name: string]: string | number | null;
};

export type TableFilter = {
  [name: string]: string | number;
};

export type RowAction = {
  name: string;
  action: () => void;
};

export type TableData = {
  columns: TableColumn[];
  rows: TableRow[];
  rowActions?: RowAction[];
  filter: TableFilter;
  sort: TableSort;
  detailPath?: string;
  detailAction?: (id: any) => void;
};
