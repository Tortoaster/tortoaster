import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import type { IProject } from "$lib/types/projects";
import type { ICommentWithUser } from "$lib/types/comments";

export const load: PageLoad = ({ params, fetch }) => {
  const project: Promise<IProject> = fetch(
    `${PUBLIC_API_URL}/projects/${params.id}`,
  ).then((response) => response.json());

  const content = fetch(`${PUBLIC_BUCKET_URL}/content/${params.id}.md`)
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  const comments: Promise<ICommentWithUser[]> = fetch(
    `${PUBLIC_API_URL}/projects/${params.id}/comments`,
  ).then((response) => response.json());

  return { project, content, comments };
};
