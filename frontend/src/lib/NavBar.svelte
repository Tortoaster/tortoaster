<script lang="ts">
    import {autoUpdate, computePosition} from "@floating-ui/dom";
    import {crossfade} from "svelte/transition";
    import {elasticOut} from "svelte/easing";

    interface Props {
        routeId?: string;
    }

    const pages = new Map([
        ['Projects', '/projects'],
        ['About', '/about'],
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
        class:rounded-b-normal={!open}
        class="lg:hidden px-single py-half text-white-bright text-xl font-bold bg-black-bright hover:bg-black-darker rounded-t-normal flex items-center gap-half transition-colors duration-200 ease-out"
        popovertarget="menu">
    <svg class="text-white-bright w-single h-single transition-transform duration-200 ease-out" class:turn={open}
         inline-src="arrow"/>
    {activeRoute}
</button>

<div bind:this={popover} ontoggle={(e) => open = e.newState === 'open'} id="menu"
     class="lg:hidden absolute bg-black-bright rounded-b-normal" popover="auto">
    <nav id="nav" class="flex flex-col last:rounded-b">
        {#each pages as [page, href]}
            <a onclick={() => popover?.hidePopover()}
               class:text-white-bright={page !== activeRoute}
               class:text-black-darkest={page === activeRoute}
               class:hover:text-white-bright={page === activeRoute}
               class:bg-white-bright={page === activeRoute}
               class="px-single py-half text-xl font-bold hover:bg-black-darker transition-colors duration-200 ease-out"
               {href}>
                {page}
            </a>
        {/each}
    </nav>
</div>

<!-- Desktop -->
<div class="hidden lg:block">
    <nav class="flex gap-single">
        {#each pages as [page, href]}
            <a onclick={() => popover?.hidePopover()} class="btn text-xl group" {href}>
                {page}
                {#if page === activeRoute}
                    <hr in:receive={{ key: 0 }}
                        out:send={{ key: 0 }}
                        class="bar mt-border -mb-[calc(2*var(--spacing-border))] w-full group-hover:before:bg-black before:transition-colors before:duration-200 before:ease-out"/>
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
