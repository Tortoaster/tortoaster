<script lang="ts">
    import {PUBLIC_BUCKET_URL} from '$env/static/public';
    import type {PageData} from './$types';
    import Throbber from "$lib/Throbber.svelte";
    import Overlay from "$lib/Overlay.svelte";
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
