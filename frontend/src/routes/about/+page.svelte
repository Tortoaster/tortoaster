<script lang="ts">
    import Overlay from "$lib/Overlay.svelte";

    const skills = new Map([
        ['Languages', new Map([
            ['Rust', 0.9],
            ['TypeScript', 0.8],
            ['JavaScript', 0.8],
            ['Kotlin', 0.7],
        ])],
        ['Frameworks', new Map([
            ['Angular', 0.9],
            ['Svelte', 0.9],
            ['UnoCSS', 0.8],
            ['Tailwind', 0.7],
        ])],
        ['CI/CD', new Map([
            ['GitHub Actions', 0.9],
            ['Docker', 0.9],
            ['Kubernetes', 0.8],
            ['Helm', 0.7],
        ])],
        ['Tools', new Map([
            ['Postgres', 0.9],
            ['S3', 0.8],
            ['Grafana', 0.6],
            ['OpenAPI', 0.6],
        ])],
    ]);
</script>

<svelte:head>
    <title>About - Tortoaster</title>
</svelte:head>

<main class="flex flex-col gap p">
    <hr class="hr"/>

    <blockquote class="prose prose-white sm:prose-sm lg:prose-lg xl:prose-xl mx-auto">
        I'm a developer with a passion for writing high quality, robust code that will stand the test of time. Using modern technologies and frameworks like Rust, TypeScript, OpenAPI, Docker and Kubernetes. From intuitive frontends to scalable backends, from future-proof documentation to extensive tests, and from cloud deployment to pro-active performance monitoring. I just enjoy making things you can rely on.
    </blockquote>

    <hr class="hr"/>

    <div class="flex gap justify-center">
        <a class="p rounded text-black-darker hover:bg-black-darker hover:text-black transition-colors" href="https://github.com/Tortoaster">G</a>
        <a class="p rounded text-black-darker hover:bg-black-darker hover:text-black transition-colors" href="https://www.linkedin.com/in/rick-van-der-wal/">L</a>
        <a class="p rounded text-black-darker hover:bg-black-darker hover:text-black transition-colors" href="mailto:rick@tortoaster.com">E</a>
    </div>
</main>

<Overlay>
    <h3 class="text-2xl text-white-bright font-bold">Work Experience</h3>
</Overlay>

<div>
    <p class="text-black-darker text-lg font-bold p w-full text-center">Nothing here yet!</p>
</div>

<Overlay>
    <h3 class="text-2xl text-white-bright font-bold">Skills</h3>
</Overlay>

<div class="p-half flex flex-wrap">
    {#each skills as [category, items]}
        <details class="p-half basis-full md:basis-1/2 lg:basis-1/3 xl:basis-1/4" open>
            <summary class="p text-lg rounded font-bold hover:bg-black-darker transition-colors">
                {category}
            </summary>
            <div class="mt flex flex-col gap-half">
                {#each items as [item, value]}
                    <div>
                        <p>{item}</p>
                        <progress class="w-full rounded h-border text-white-bright bg-black-darker [&::-webkit-progress-bar]:rounded [&::-webkit-progress-value]:rounded [&::-webkit-progress-bar]:bg-black-darker [&::-webkit-progress-value]:bg-white-bright [&::-moz-progress-bar]:bg-white-bright" max="1" {value}></progress>
                    </div>
                {/each}
            </div>
        </details>
    {/each}
</div>
