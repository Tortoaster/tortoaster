<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        children?: Snippet;
        class?: any;
    }

    let {children, class: className = ''}: Props = $props();
</script>

<div class="flex">
    <div class:border-t-single={true} class:w-single={true} class:h-single={true} class:-mr-border-plus={true}
         class:z-10={true} class={className}>
    </div>
    <div class="card basis-full">
        {@render children?.()}
    </div>
</div>
