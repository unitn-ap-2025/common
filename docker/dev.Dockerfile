# syntax=docker/dockerfile

FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive LANG=C.UTF-8 LC_ALL=C.UTF-8

ARG APT_PROFILES="base c"

COPY ops/apt /ops/apt

RUN find /ops/apt -type f -name '*.txt' -exec sed -i 's/\r$//' {} + && \
    for f in $(echo $APT_PROFILES | tr ' ' '\n' | sed 's|^|/ops/apt/|;s|$|.txt|'); do test -f "$f" && cat "$f"; done | \
    awk '{sub(/#.*/,""); print}' | sed '/^\s*$/d' | sort -u > /ops/all.txt

RUN --mount=type=cache,target=/var/cache/apt --mount=type=cache,target=/var/lib/apt apt-get update && xargs -r -a /ops/all.txt apt-get install -y --no-install-recommends && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && /opt/cargo/bin/rustup default stable && /opt/cargo/bin/rustup component add rustfmt clippy

WORKDIR /workspace

RUN git config --system --add safe.directory /workspace

CMD ["bash","-lc","exec sleep infinity"]
