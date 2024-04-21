<script lang="ts">
  import Table from "$components/atoms/dateTable/Table.svelte";
  import TableBody from "$components/atoms/dateTable/TableBody.svelte";
  import TableHead from "$components/atoms/dateTable/TableHead.svelte";
  import TableHeadCell from "$components/atoms/dateTable/TableHeadCell.svelte";
  import DataTable from "$components/molecules/DataTable.svelte";
  import type { TableColumn, TableData, TableRow } from "$domain";
  import { displayLocaleDateTime, displayTimeFromSeconds } from "$lib";
  import PaymentListRow from "./PaymentListRow.svelte";

  export let rows: TableRow[];
  export let reloadPayments: () => void;

  const columns: TableColumn[] = [
    {
      name: "Payed at",
      field: "payedAt",
      displayHelper: displayLocaleDateTime,
    },
    {
      name: "Amount",
      field: "amount",
    },
    {
      name: "Payment method",
      field: "paymentMethod",
    },
    {
      name: "Status",
      field: "status",
    },
  ];
</script>

<Table>
  <TableHead>
    {#each columns as column}
      <TableHeadCell>{column.name}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody>
    {#each rows as row, index}
      <PaymentListRow {columns} {row} {reloadPayments} />
    {/each}
  </TableBody>
</Table>
