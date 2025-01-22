<script lang="ts">
    import {autoUpdate, computePosition} from "@floating-ui/dom";
    import {crossfade, fly} from "svelte/transition";
    import {elasticOut} from "svelte/easing";

    interface Props {
        routeId?: string;
    }

    const pages = new Map([
        ['Projects', '/projects'],
        ['Experience', '/experience'],
    ]);

    let button: HTMLButtonElement;
    let popover: HTMLDivElement;

    let {routeId}: Props = $props();

    let open = $state(false);
    let activeRoute = $derived([...pages.entries()].find(([_, page]) => routeId?.startsWith(page))?.[0]);

    const [send, receive] = crossfade({
        duration: 400,
        easing: elasticOut,
    });

    $effect(() => {
        if (open) {
            let width = Math.max(button.offsetWidth, popover.offsetWidth);
            Object.assign(button.style, {width: `${width}px`});
            Object.assign(popover.style, {width: `${width}px`});
        } else {
            Object.assign(button.style, {width: null});
            Object.assign(popover.style, {width: null});
        }
    });

    $effect(() => autoUpdate(
        button,
        popover,
        () => computePosition(button, popover, {placement: "bottom-start"}).then(({x, y}) => {
            if (popover) {
                Object.assign(popover.style, {
                    top: `${y}px`,
                    left: `${x}px`
                });
            }
        }),
    ));
</script>

{#snippet navbar(underline)}
    <nav id="nav" class="flex flex-col lg:flex-row lg:gap">
        {#each pages as [page, href]}
            <a onclick={() => popover?.hidePopover()}
               class:text-white-bright={page !== activeRoute}
               class:text-black-darkest={page === activeRoute}
               class:hover:text-white-bright={page === activeRoute}
               class:bg-white-bright={page === activeRoute}
               class="p-half lg:p lg:rounded text-xl lg:text-white-bright font-bold lg:bg-transparent hover:bg-black-darker transition-colors relative group"
               {href}>
                {page}
                {#if underline && page === activeRoute}
                    <div
                            in:receive={{ key: 0 }}
                            out:send={{ key: 0 }}
                            class="absolute bottom-half w-[calc(100%-2*var(--spacing))] h-border rounded bg-black-darker group-hover:bg-black transition-colors"></div>
                {/if}
            </a>
        {/each}
    </nav>
{/snippet}

<!-- Mobile -->
<button bind:this={button}
        class:rounded-b={!open}
        class="lg:hidden p-half text-white-bright text-xl font-bold bg-black-bright hover:bg-black-darker transition-colors rounded-t flex items-center gap-half"
        popovertarget="menu">
    <svg class="text-white-bright w-spacing h-spacing transition-transform" class:turn={open}
         inline-src="arrow"/>
    {activeRoute}
</button>

<div bind:this={popover} ontoggle={(e) => open = e.newState === 'open'} id="menu"
     class="lg:hidden absolute m-0 p-0 bg-black-bright rounded-b" popover="auto">
    {@render navbar(false)}
</div>

<!-- Desktop -->
<div class="hidden lg:block">
    {@render navbar(true)}
</div>

<style>
    .turn {
        transform: rotateY(180deg) rotateZ(90deg);
    }

    #nav a:last-child {
        border-bottom-left-radius: var(--border-radius);
        border-bottom-right-radius: var(--border-radius);
    }
</style>
