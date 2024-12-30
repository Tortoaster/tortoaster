<script lang="ts">
    import '../app.css';
    import Header from "$lib/Header.svelte";
    import Card from "$lib/Card.svelte";
    import Footer from "$lib/Footer.svelte";
    import NavBar from "$lib/NavBar.svelte";

    let {children} = $props();
</script>

<div class="bg-background min-h-screen overflow-x-clip">
    <div class="container mx-auto sm:px">
        <Header/>
        <Card class="bg-black" extend>
            <div class="flex justify-between items-center p">
                <NavBar/>
                <a aria-label="Home" class="basis-1/2 sm:basis-1/3 lg:basis-1/4" href="/">
                    <svg class="w-full text-black-darker" inline-src="logo"/>
                </a>
            </div>
            {@render children()}
        </Card>
        <Footer/>
    </div>
</div>
