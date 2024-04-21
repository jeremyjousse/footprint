<script lang="ts">
  export let dateTime: string;

  let date = new Date(dateTime);
  let formattedDate = date.toLocaleString("fr-FR", { timeZone: "UTC" });
</script>

{formattedDate}
