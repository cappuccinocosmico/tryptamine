FROM rust:alpine as build-image
WORKDIR /app
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN chmod +x ./tailwindcss-linux-x64
COPY . .
RUN ./tailwindcss-linux-x64 -i styles/input.css -o styles/output.css --minify
RUN cargo build --release
FROM alpine:3.20 as final-image
WORKDIR /app
COPY --from=build-image /app/target/release/tryptamine .
COPY --from=build-image /app/styles/output.css ./styles/output.css
COPY --from=build-image /app/assets/ ./assets/ 
COPY --from=build-image /app/markdown/ ./markdown/
EXPOSE 3003
CMD [ "./tryptamine" ]
