# syntax=docker/dockerfile:1

FROM rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

WORKDIR /source
COPY Cargo.toml Cargo.lock ./
COPY README.md ./
COPY bench ./bench
COPY examples ./examples
COPY src ./src
RUN --mount=type=cache,id=tui-textarea-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=tui-textarea-recording-target,target=/source/target \
    cargo build --locked --release --examples --features search && \
    mkdir /artifacts && \
    cp target/release/examples/minimal \
       target/release/examples/editor \
       target/release/examples/single_line \
       target/release/examples/split \
       target/release/examples/variable \
       target/release/examples/vim \
       target/release/examples/popup_placeholder \
       target/release/examples/password \
       target/release/examples/wrap \
       target/release/examples/undo_coalescing \
       /artifacts/

FROM ghcr.io/charmbracelet/vhs:latest@sha256:9d5fc3dc0c160b0fb1d2212baff07e6bdf3fa9438c504a3237484567302fcf93

COPY --from=builder /artifacts/minimal /usr/local/bin/tui-textarea-minimal
COPY --from=builder /artifacts/editor /usr/local/bin/tui-textarea-editor
COPY --from=builder /artifacts/single_line /usr/local/bin/tui-textarea-single-line
COPY --from=builder /artifacts/split /usr/local/bin/tui-textarea-split
COPY --from=builder /artifacts/variable /usr/local/bin/tui-textarea-variable
COPY --from=builder /artifacts/vim /usr/local/bin/tui-textarea-vim
COPY --from=builder /artifacts/popup_placeholder /usr/local/bin/tui-textarea-popup-placeholder
COPY --from=builder /artifacts/password /usr/local/bin/tui-textarea-password
COPY --from=builder /artifacts/wrap /usr/local/bin/tui-textarea-wrap
COPY --from=builder /artifacts/undo_coalescing /usr/local/bin/tui-textarea-undo-coalescing
COPY recordings/fixtures/editor.txt /opt/tui-textarea/editor.txt
COPY recordings/record.sh /usr/local/bin/record-examples
RUN chmod 0666 /opt/tui-textarea/editor.txt && chmod +x /usr/local/bin/record-examples

ENTRYPOINT ["/usr/local/bin/record-examples"]
