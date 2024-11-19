interface Props {
  mood?: "Happy" | "Sad";
}

export function Header({ mood = "Happy" }: Props) {
  return (
    <header class="w-1/3 aspect-[2/1] relative">
      <a hx-boost="true" href="/">
        {mood === "Happy"
          ? (
            <>
              <img
                alt="Tortoaster logo"
                src="/turtle-back.svg"
                class="absolute w-full object-cover"
              />
              <img
                alt="Tortoaster logo"
                src="/turtle-front.svg"
                class="absolute z-50 object-cover w-full"
              />
            </>
          )
          : (
            <>
              <img
                alt="Tortoaster logo"
                src="/turtle-back-concerned.svg"
                class="absolute w-full object-cover"
              />
              <img
                alt="Tortoaster logo"
                src="/turtle-front.svg"
                class="absolute z-50 object-cover w-full"
              />
            </>
          )}
      </a>
    </header>
  );
}
