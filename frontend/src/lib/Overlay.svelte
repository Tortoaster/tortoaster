<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        children?: Snippet;
    }

    let {children}: Props = $props();
</script>

<div class="card-y sm:card sm:-mx-single bg-cyan px-single sm:px-double py-single z-10">
    {@render children?.()}
</div>
