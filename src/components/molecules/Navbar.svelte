<script>
  import NavbarItem from "$components/atoms/NavbarItem.svelte";
</script>

<nav class="hidden lg:mt-10 lg:block">
  <ul role="list">
    <NavbarItem icon="Patient" text="Patients" link="/patients" />
    <NavbarItem icon="Settings" text="Settings" link="/settings" />
  </ul>
</nav>
