<script lang="ts">
  import { superForm } from "sveltekit-superforms/client";
  import { zod } from "sveltekit-superforms/adapters";

  import { t } from "$i18n";
  import { consultationTypeSchema, type ConsultationType } from "$domain";
  import FormInput from "$components/atoms/form/FormInput.svelte";
  import Button from "$components/atoms/Button.svelte";
  import { goto } from "$app/navigation";
  import { consultationTypeService } from "$services";

  export let consultationType: ConsultationType;
  export let action: "add" | "update";

  const { form, enhance, errors, validateForm } = superForm(consultationType, {
    validators: zod(consultationTypeSchema),
    dataType: "json",
  });

  const handleUpdateConsultationType = async () => {
    const result = await validateForm();
    if (result.valid) {
      validateForm();
      if (action === "add") {
        const addOk = await consultationTypeService.add($form);

        if (addOk) {
          goto("/settings/consultation-types");
        }
      } else {
        const addOk = await consultationTypeService.update($form);
      }
    }
  };
</script>

<form
  method="POST"
  use:enhance
  on:submit|preventDefault={handleUpdateConsultationType}
  class="mt-8"
>
  {$errors.length}
  <div class="grid grid-cols-5 gap-4">
    <div>{$t("consultationTypes.consultationType.name")}</div>
    <div class="col-span-2">
      <FormInput
        type="text"
        name="name"
        placeholder={$t("consultationTypes.consultationType.name")}
        bind:value={$form.name}
        errors={$errors.name}
      />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-4 mt-4">
    <div>{$t("consultationTypes.consultationType.price")}</div>
    <div class="col-span-2">
      <FormInput
        name="price"
        type="number"
        placeholder={$t("consultationTypes.consultationType.price")}
        bind:value={$form.price}
        errors={$errors.price}
      />
    </div>
  </div>

  <div class="flex justify-end">
    <Button color="primary" type="submit"
      >{$t(`consultationTypes.${action}.buttonAction`)}</Button
    >
  </div>
</form>
