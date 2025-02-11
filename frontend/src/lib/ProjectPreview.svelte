<script lang="ts">
    import {formatDistanceToNow} from "date-fns";
    import type {IProjectPreview} from "$lib/types/projects";

    interface Props {
        project: IProjectPreview;
        bucketUrl: string;
    }

    let {project, bucketUrl}: Props = $props();
</script>

<div class="card bg-black-bright">
    <a href="/projects/{ project.id }" class="group">
        <img class="bg-black object-cover group-hover:object-contain aspect-card w-full"
             alt="{ project.name } thumbnail"
             src="{ bucketUrl }/thumbnails/{ project.thumbnailId }"/>
        <div class="overflow-hidden p-single aspect-card flex flex-col gap-single">
            <div class="flex justify-between items-center">
                <h3 class="text-xl font-bold">{ project.name }</h3>
                <small class="text-gray-bright text-right">{ formatDistanceToNow(project.datePosted, {addSuffix: true}) }</small>
            </div>
            <div class="description">{ project.preview }</div>
        </div>
        <div class="absolute w-full bottom-0 -translate-y-full h-double bg-linear-to-t from-black-bright from-50%"></div>
    </a>
    <span class="bite"></span>
</div>
