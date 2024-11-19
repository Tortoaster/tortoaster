import { Head } from "$fresh/runtime.ts";
import { PageProps } from "$fresh/server.ts";

export default function Greet(props: PageProps) {
    return <>
        <Head>
            <title>Projects - Tortoaster</title>
        </Head>
        <div>Projects</div>
        </>;
}


