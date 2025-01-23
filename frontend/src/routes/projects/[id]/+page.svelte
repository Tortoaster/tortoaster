<script lang="ts">
    import Card from "$lib/Card.svelte";
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
    <div class="flex justify-between items-center">
        <h1 class="text-4xl text-white-bright font-bold">{data.project.name}</h1>
        <div class="flex">
            <span class="text-white italic hidden lg:block">{day},&nbsp;</span>
            <span class="text-white italic">{date}</span>
            <span class:hidden={+year !== new Date().getFullYear()} class="text-white italic">,&nbsp;{year}</span>
        </div>
    </div>
</Overlay>

<div class="flex flex-col gap p">
    <Card class="max-w-fit m-auto">
        <img alt="Thumbnail" src="{PUBLIC_BUCKET_URL}/thumbnails/{data.project.thumbnailId}">
    </Card>

    {#if data.project.projectUrl}
        <a class="btn btn-black text-lg mx-auto" href={data.project.projectUrl}>Visit project page &#x2197;</a>
    {/if}

    <div class="prose prose-white sm:prose-sm lg:prose-lg xl:prose-xl mx-auto">
        {#await data.content}
            <Throbber class="text-black-darker"/>
        {:then content}
            {@html content}
        {/await}
    </div>
</div>

<Overlay>
    <h3 class="text-2xl text-white-bright font-bold">Comments</h3>
</Overlay>

{#await data.comments}
    <Throbber class="text-black-darker"/>
{:then comments}
    <div class="prose prose-white sm:prose-sm lg:prose-lg xl:prose-xl mx-auto flex flex-col gap p">
        <form class="flex flex-col gap-half">
            <SpeechBubble class="bg-gray-bright hover:bg-foreground transition-colors">
                <textarea
                        class="bg-gray-bright focus:bg-foreground transition-colors text-black-darkest w-full outline-none p"></textarea>
            </SpeechBubble>

            <div class="flex justify-end">
                <Card>
                    <button class="px py-half bg-cyan text-white-bright hover:bg-white-bright hover:text-cyan transition-colors"
                            type="submit">Comment
                    </button>
                </Card>
            </div>
        </form>

        {#each comments as comment}
            {JSON.stringify(comment)}
        {:else}
            <p class="text-black-darker text-lg font-bold w-full text-center">Nothing here yet!</p>
        {/each}
    </div>
{/await}
