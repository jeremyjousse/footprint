<script lang="ts">
  import Button from "$components/atoms/Button.svelte";
  import TableBody from "$components/atoms/dateTable/TableBody.svelte";
  import TableBodyCell from "$components/atoms/dateTable/TableBodyCell.svelte";
  import TableBodyRow from "$components/atoms/dateTable/TableBodyRow.svelte";
  import { DETAIL_ACTIONS, type TableColumn, type TableRow } from "$domain";
  import { displayTableRowValue } from "$lib";
  import PaymentForm from "./PaymentForm.svelte";

  export let row: TableRow;
  export let columns: TableColumn[];
  export let reloadPayments: () => void;

  let editing: boolean = false;

  const handleEditing = () => {
    editing = true;
  };

  const handleSave = () => {
    editing = false;
  };
</script>

{#if editing}
  <PaymentForm
    payment={row}
    bind:editing
    action={DETAIL_ACTIONS.Edit}
    {reloadPayments}
  />
{:else}
  <TableBodyRow on:click={() => handleEditing()}>
    {#each columns as column}
      <TableBodyCell
        >{displayTableRowValue(row[column.field], column)}</TableBodyCell
      >
    {/each}
  </TableBodyRow>
{/if}
