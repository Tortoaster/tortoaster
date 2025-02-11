<script lang="ts">
    import '../app.css';
    import Header from "$lib/Header.svelte";
    import Footer from "$lib/Footer.svelte";
    import NavBar from "$lib/NavBar.svelte";
    import {page} from '$app/state';
    import type {Snippet} from "svelte";

    interface Props {
        children: Snippet,
    }

    let {children}: Props = $props();
</script>

<div class="font-display text-white-bright bg-background min-h-screen overflow-x-clip">
    <div class="container mx-auto sm:px-single">
        <Header mood={page.error ? 'sad' : 'happy'}/>
        <div class="card-y sm:card bg-black">
            <div class="flex flex-col gap-half sm:flex-row-reverse justify-between items-center p-single">
                <a aria-label="Home" class="basis-1/2 sm:basis-1/3 lg:basis-1/4" href="/">
                    <svg class="hidden sm:block w-full text-black-darker" inline-src="logo"/>
                    <svg class="sm:hidden w-full text-black-darker" inline-src="logo-text"/>
                </a>
                <NavBar routeId={page.route.id ?? undefined}/>
            </div>
            {@render children()}
        </div>
        <Footer/>
    </div>
</div>
