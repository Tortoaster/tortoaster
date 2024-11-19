import { PageProps } from "$fresh/server.ts";
import { Partial } from "https://deno.land/x/fresh@1.6.8/src/runtime/Partial.tsx";
import { Header } from "../components/Header.tsx";
import { Footer } from "../components/Footer.tsx";
import { Logo } from "../icons/Logo.tsx";

export default function Layout({ Component, state }: PageProps) {
  return (
    <div class="bg-blue-300 min-h-screen font-comic">
      <div f-client-nav class="flex flex-col items-center container mx-auto">
        <Partial name="header">
          <Header />
        </Partial>
        <div class="flex flex-col w-full px-8 py-16 gap-8 bg-stone-100 border-4 border-stone-900 shadow-comic relative">
          <div class="flex justify-between items-center md:px-12">
            <Partial name="nav">
              <nav class="flex">
              </nav>
            </Partial>
            <a href="/" class="block text-stone-300 h-32">
              <Logo/>
            </a>
          </div>
          <main>
            <Partial name="main">
              <Component />
            </Partial>
          </main>
          <div class="absolute bottom-0 right-0 h-16 mt-16 aspect-square bg-[linear-gradient(to_top_left,_#93c5fd_50%,_#1c1917_50%,_#1c1917_calc(50%_+_8px),_#d6d3d1_calc(50%_+_8px))] border-t-4 border-t-stone-900 border-l-stone-900 border-l-4 mr-[-8px] mb-[-8px] transition-all">
          </div>
        </div>
        <Footer />
      </div>
    </div>
  );
}
