<script lang="ts">
    import {autoUpdate, computePosition} from "@floating-ui/dom";

    let button: HTMLButtonElement;
    let popover: HTMLDivElement;

    let open = $state(false);

    $effect(() => {
        if (open) {
            Object.assign(button.style, {width: `${popover.offsetWidth}px`});
        } else {
            Object.assign(button.style, {width: null});
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

{#snippet navbar()}
    <nav class="flex flex-col lg:flex-row lg:gap">
        <a onclick={() => popover?.hidePopover()}
           class="p lg:rounded text-white-bright hover:bg-black-darker transition-colors"
           href="/projects">Projects</a>
        <a onclick={() => popover?.hidePopover()}
           class="p lg:rounded text-white-bright hover:bg-black-darker transition-colors"
           href="/experience">Experience</a>
        <a onclick={() => popover?.hidePopover()}
           class="p rounded-b lg:rounded text-white-bright hover:bg-black-darker transition-colors"
           href="/#">Contact</a>
    </nav>
{/snippet}

<button bind:this={button}
        class:rounded-b={!open}
        class="lg:hidden p text-white-bright bg-black-bright hover:bg-black-darker transition-colors rounded-t"
        popovertarget="menu">
    {open ? 'x' : '='}
</button>
<div bind:this={popover} ontoggle={(e) => open = e.newState === 'open'} id="menu"
     class="lg:hidden absolute m-0 p-0 bg-black-bright rounded-b" popover="auto">
    {@render navbar()}
</div>

<div class="hidden lg:block">
    {@render navbar()}
</div>

<style>
    #menu::backdrop {
        margin: 0;
    }
</style>
