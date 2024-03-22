import { toSnakeCase } from "$lib/helpers";

export type TableColumnFilter = {
  type: "string" | "boolean" | "list"; // TODO enum
  values?: string[];
};

// TODO add an optional formatting function
export type TableColumn = {
  name: string;
  field: string;
  displayHelper?: () => string;
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

// class TableFilter {

// }

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
  [name: string]: string | number;
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
};
