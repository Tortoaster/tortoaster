import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import type { Page } from "$lib/types/pagination";
import type { IProjectPreview } from "$lib/types/projects";

export const load: PageLoad = async ({ fetch }) => {
  const about = await fetch(PUBLIC_BUCKET_URL + "/system/projects.md")
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  const projects: Page<IProjectPreview> = await fetch(
    PUBLIC_API_URL + "/projects",
  ).then((response) => response.json());

  return { about, projects };
};
