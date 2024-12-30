<script lang="ts">
    import Card from "$lib/Card.svelte";
    import type {Snippet} from "svelte";

    interface Props {
        children?: Snippet;
    }

    let {children}: Props = $props();
</script>

<Card class="sm:-mx bg-cyan px sm:px-double py z-10" extend>
    {@render children?.()}
</Card>
