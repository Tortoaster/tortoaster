<script lang="ts">
    import ProjectPreview from "$lib/ProjectPreview.svelte";
    import {PUBLIC_BUCKET_URL} from '$env/static/public';
    import type {PageData} from './$types';
    import Overlay from "$lib/Overlay.svelte";

    interface Props {
        data: PageData;
    }

    const {data}: Props = $props();
</script>

<svelte:head>
    <title>Projects - Tortoaster</title>
</svelte:head>

<Overlay>
    <div class="md">
        {@html data.about}
    </div>
</Overlay>

<main class="flex flex-wrap p-half">
    {#each data.projects.items as project}
        <div class="basis-full md:basis-1/2 lg:basis-1/3 xl:basis-1/4 p-half">
            <ProjectPreview {project} bucketUrl={PUBLIC_BUCKET_URL}/>
        </div>
    {:else}
        <p class="text-black-darker text-lg font-bold p-single w-full text-center">Nothing here yet!</p>
    {/each}
</main>
