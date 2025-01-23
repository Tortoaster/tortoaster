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
        ['Contact', '/contact'],
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
    <nav id="nav" class="flex flex-col last:rounded-b">
        {#each pages as [page, href]}
            <a onclick={() => popover?.hidePopover()}
               class:text-white-bright={page !== activeRoute}
               class:text-black-darkest={page === activeRoute}
               class:hover:text-white-bright={page === activeRoute}
               class:bg-white-bright={page === activeRoute}
               class="p-half text-xl font-bold hover:bg-black-darker transition-colors"
               {href}>
                {page}
            </a>
        {/each}
    </nav>
</div>

<!-- Desktop -->
<div class="hidden lg:block">
    <nav class="flex gap">
        {#each pages as [page, href]}
            <a onclick={() => popover?.hidePopover()} class="btn btn-black text-xl relative group" {href}>
                {page}
                {#if page === activeRoute}
                    <hr in:receive={{ key: 0 }}
                        out:send={{ key: 0 }}
                        class="hr absolute bottom-half w-[calc(100%-2*var(--spacing))] group-hover:before:bg-black transition-colors"/>
                {/if}
            </a>
        {/each}
    </nav>
</div>

<style>
    .turn {
        transform: rotateY(180deg) rotateZ(90deg);
    }
</style>
