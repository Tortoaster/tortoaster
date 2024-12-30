<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        bite?: boolean;
        // If enabled, the sides of the card will not be displayed on small screens
        extend?: boolean;
        children?: Snippet;
        class?: any;
    }

    let {bite = false, extend = false, children, class: className = ''}: Props = $props();
</script>

<div class:card={true} class:border-y={true} class:border-x={!extend} class:sm:border-x={true} class={className}>
    {@render children?.()}
    {#if bite}
        <span class="bite"></span>
    {/if}
</div>

<style>
    .card {
        position: relative;
    }

    .card::before {
        content: "";
        position: absolute;
        left: calc(-1 * var(--border-thickness));
        bottom: calc(-2 * var(--border-thickness) + 1pt);
        width: calc(100% + 3 * var(--border-thickness) - 1pt);
        height: 0;
        border: var(--border-thickness) solid var(--darkest-black);
        border-left-color: transparent;
        border-bottom-width: 0;
    }

    .card::after {
        content: "";
        position: absolute;
        top: calc(-1 * var(--border-thickness));
        right: calc(-2 * var(--border-thickness) + 1pt);
        width: 0;
        height: calc(100% + 3 * var(--border-thickness) - 1pt);
        border: var(--border-thickness) solid var(--darkest-black);
        border-top-color: transparent;
        border-right-width: 0;
    }

    .bite {
        background-color: var(--black);
        border: var(--border-thickness) solid var(--darkest-black);
        border-top-width: 0;
        border-radius: 0 0 9999px 9999px;
        position: absolute;
        top: calc(-1 * var(--border-thickness));
        left: 50%;
        transform: translateX(-50%);
        aspect-ratio: 2 / 1;
        height: var(--spacing);
    }

    .bite::after {
        content: "";
        position: absolute;
        width: 50%;
        height: calc(100% + var(--border-thickness) - 1pt);

        border: var(--border-thickness) solid var(--darkest-black);
        border-left-width: var(--border-thickness);
        border-top-color: var(--black);
        border-bottom-width: calc(var(--border-thickness) - 1pt);
        border-right-width: 0;
        border-bottom-left-radius: 9999px;
    }
</style>
