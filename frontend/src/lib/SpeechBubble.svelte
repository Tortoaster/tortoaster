<script lang="ts">
    import type {Snippet} from "svelte";
    import Card from "$lib/Card.svelte";

    interface Props {
        children?: Snippet;
        class?: any;
    }

    let {children, class: className = ''}: Props = $props();
</script>

<div class="flex">
    <div class:border-t={true} class:w-spacing={true} class:h-spacing={true} class:-mr-border-plus={true}
         class:z-10={true} class={className}>

    </div>
    <Card class="basis-full">
        {@render children?.()}
    </Card>
</div>
