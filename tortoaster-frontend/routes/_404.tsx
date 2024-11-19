import { Head } from "$fresh/runtime.ts";

export default function Error404() {
  return <>
      <Head>
          <title>Not found - Tortoaster</title>
      </Head>
      <div class="prose prose-stone mx-auto">
          <h1>404 Not found</h1>
          <p>That page doesn't seem to exist anymore</p>
      </div>
  </>;
}
