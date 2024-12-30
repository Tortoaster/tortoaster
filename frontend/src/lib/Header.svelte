<script lang="ts">
    interface Props {
        mood?: 'happy' | 'sad';
    }

    let {mood = 'happy'}: Props = $props();
</script>

<header class="w-1/3 aspect-2/1 mx-auto relative text-black-darkest">
    {#if mood === 'happy'}
        <svg inline-src="turtle" preserveAspectRatio="xMinYMin slice" class="absolute"/>
    {:else}
        <svg inline-src="turtle-sad" preserveAspectRatio="xMinYMin slice" class="absolute"/>
    {/if}
    <svg inline-src="turtle-mouth" class="absolute z-50"/>
</header>
