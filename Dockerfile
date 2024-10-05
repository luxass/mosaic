FROM bitnami/minideb:bookworm

WORKDIR /app

COPY target/release/mosaic /app/mosaic

RUN install_packages ca-certificates curl && \
      chmod +x /app/mosaic

ENV MOSAIC_LISTEN_PORT=3939
EXPOSE $MOSAIC_LISTEN_PORT

ENTRYPOINT [ "/app/mosaic" ]

STOPSIGNAL SIGTERM