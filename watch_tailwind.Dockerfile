FROM rust:latest

RUN cargo install cargo-watch
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN chmod +x tailwindcss-linux-x64
RUN mv tailwindcss-linux-x64 /usr/local/bin/tailwind

ENTRYPOINT ["cargo", "watch", "-x", "run", "-w", "src", "-w", "templates", "-w", "input.css", "-w", "tailwind.config.js"]
