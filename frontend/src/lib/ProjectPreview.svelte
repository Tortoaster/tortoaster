<script lang="ts">
    import {formatDistanceToNow} from "date-fns";
    import Card from "$lib/Card.svelte";
    import type {IProjectPreview} from "$lib/types/projects";

    interface Props {
        project: IProjectPreview;
        bucketUrl: string;
    }

    let {project, bucketUrl}: Props = $props();
</script>

<Card bite class="bg-black-bright">
    <a href="/projects/{ project.id }" class="group">
        <div class="relative">
            <img class="bg-black object-cover group-hover:object-contain aspect-3/2 w-full"
                 alt="{ project.name } thumbnail"
                 src="{ bucketUrl }/thumbnails/{ project.thumbnailId }"/>
            <div class="overflow-hidden p aspect-3/2 flex flex-col gap">
                <div class="flex justify-between items-center">
                    <h3 class="text-white-bright text-xl font-bold">{ project.name }</h3>
                    <small class="text-gray-bright text-right">{ formatDistanceToNow(project.datePosted, {addSuffix: true}) }</small>
                </div>
                <div class="description">{ project.preview }</div>
            </div>
        </div>
        <div class="absolute w-full bottom-0 p bg-gradient-to-t from-black-bright from-50%"></div>
    </a>
</Card>
