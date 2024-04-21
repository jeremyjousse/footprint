<script lang="ts">
  import type { TableData, TableRow } from "$domain/valueObjects/TableData";
  import { goto } from "$app/navigation";
  import Table from "$components/atoms/dateTable/Table.svelte";
  import TableHead from "$components/atoms/dateTable/TableHead.svelte";
  import TableHeadCell from "$components/atoms/dateTable/TableHeadCell.svelte";
  import TableBody from "$components/atoms/dateTable/TableBody.svelte";
  import TableBodyCell from "$components/atoms/dateTable/TableBodyCell.svelte";
  import TableBodyRow from "$components/atoms/dateTable/TableBodyRow.svelte";
  import { DETAIL_ACTIONS } from "$domain";
  import { displayTableRowValue } from "$lib";

  export let data: TableData;

  const handleDetail = (id: any) => {
    if (data.detailAction) {
      data.detailAction(id);
    } else {
      goto(`${data.detailPath}?id=${id}&action=${DETAIL_ACTIONS.View}`);
    }
  };
</script>

<Table>
  <TableHead>
    {#each data.columns as column}
      <TableHeadCell>{column.name}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody>
    {#each data.rows as row, index}
      <TableBodyRow onclick={() => handleDetail(row.id)}>
        {#each data.columns as column}
          <TableBodyCell
            >{displayTableRowValue(row[column.field], column)}</TableBodyCell
          >
        {/each}
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
