<script lang="ts">
    interface Props {
        class?: any;
    }

    let {class: className = ''}: Props = $props();
</script>

<div class="size-full p-single">
    <svg class:mx-auto={true} class:mt-12={true} class:origin-[bottom_center]={true} class:animate-toaster={true} class={className} width="6rem" height="6rem"
         inline-src="bread.svg">
    </svg>
</div>
