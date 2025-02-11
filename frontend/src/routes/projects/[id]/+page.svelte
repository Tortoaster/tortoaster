<script lang="ts">
    import {PUBLIC_BUCKET_URL} from '$env/static/public';
    import type {PageData} from './$types';
    import Throbber from "$lib/Throbber.svelte";
    import Overlay from "$lib/Overlay.svelte";
    import SpeechBubble from "$lib/SpeechBubble.svelte";
    import {format} from "date-fns";

    interface Props {
        data: PageData;
    }

    const {data}: Props = $props();

    let [day, date, year] = format(data.project.datePosted, 'PPPP').split(', ');
</script>

<svelte:head>
    <title>{data.project.name} - Tortoaster</title>
</svelte:head>

<Overlay>
    <div class="flex justify-between items-baseline">
        <h1 class="text-4xl text-white-bright font-bold">{data.project.name}</h1>
        <div class="flex">
            <span class="text-white italic hidden lg:block">{day},&nbsp;</span>
            <span class="text-white italic">{date}</span>
            <span class:hidden={+year !== new Date().getFullYear()} class="text-white italic">,&nbsp;{year}</span>
        </div>
    </div>
</Overlay>

<main class="flex flex-col gap-single p-single">
    <div class="card mx-auto">
        <img alt="Thumbnail" src="{PUBLIC_BUCKET_URL}/thumbnails/{data.project.thumbnailId}">
    </div>

    {#if data.project.projectUrl}
        <a class="btn text-lg mx-auto" href={data.project.projectUrl}>Visit project page &#x2197;</a>
    {/if}

    <div class="md lg:max-w-gratio lg:mx-auto">
        {#await data.content}
            <Throbber class="text-black-darker"/>
        {:then content}
            {@html content}
        {/await}
    </div>
</main>

<Overlay>
    <h3 class="text-2xl text-white-bright font-bold">Comments</h3>
</Overlay>

{#await data.comments}
    <Throbber class="text-black-darker"/>
{:then comments}
    <div>
        <form class="flex flex-col gap-half">
            <SpeechBubble class="bg-gray-bright hover:bg-foreground transition-colors duration-200 ease-out">
                <textarea
                        class="bg-gray-bright focus:bg-foreground text-black-darkest w-full outline-hidden p-single transition-colors duration-200 ease-out"></textarea>
            </SpeechBubble>

            <div class="flex justify-end">
                <div class="card">
                    <button class="px-single py-half bg-cyan text-white-bright hover:bg-white-bright hover:text-cyan transition-colors duration-200 ease-out"
                            type="submit">Comment
                    </button>
                </div>
            </div>
        </form>

        {#each comments as comment}
            {JSON.stringify(comment)}
        {:else}
            <p class="text-black-darker text-lg font-bold w-full text-center">Nothing here yet!</p>
        {/each}
    </div>
{/await}
