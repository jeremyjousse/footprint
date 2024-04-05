<script lang="ts">
  import type { Sort } from "$domain";
  import type { TableData, TableRow } from "$domain/valueObjects/DataTable";
  import TableSortIndicator from "$components/atoms/TableSortIndicator.svelte";
  import DropdownMenuButtonGroup from "$components/molecules/DropdownMenuButtonGroup.svelte";
  import ButtonGroup from "$components/molecules/ButtonGroup.svelte";
  import { displayRowValue } from "$lib/helpers/dataTable";
  import { goto } from "$app/navigation";

  export let data: TableData;

  const handleDetail = (id: string) => {
    goto(`${data.detailPath}?id=${id}`);
  };
</script>

<table class="relative w-full table-auto cursor-default">
  <thead>
    <tr>
      {#each data.columns as column}
        <th class="sticky top-6 border-l bg-neutral-800 px-6 font-medium"
          >{column.name}
        </th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each data.rows as row, index}
      <tr
        class="{index % 2
          ? ' bg-neutral-700'
          : 'bg-zinc-800'} hover:bg-gray-600"
        on:click={() => handleDetail(row.id)}
      >
        {#each data.columns as column}
          <td>{row[column.field]}</td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
