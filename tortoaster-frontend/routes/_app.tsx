import { type PageProps } from "$fresh/server.ts";
export default function App({ Component }: PageProps) {
  return (
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Tortoaster</title>
        <link rel="stylesheet" href="/styles.css" />
        <link href="/favicon.gif" rel="icon" />
      </head>
      <body>
        <Component />
      </body>
    </html>
  );
}
