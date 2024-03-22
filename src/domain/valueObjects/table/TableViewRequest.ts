import type { Pagination } from "./Pagination";
import type { Sort } from "./Sort";

export type TableViewRequest = {
  filter: any;
  pagination: Pagination;
  sort: Sort;
};
