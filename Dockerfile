FROM rust:1
WORKDIR /app
EXPOSE 58000

# Define build arguments for UID and GID
ARG USER_ID
ARG GROUP_ID

# Create a group and user inside the container with matching IDs
RUN groupadd --gid $GROUP_ID hostgroup && \
  useradd --uid $USER_ID --gid $GROUP_ID --create-home hostuser

# Switch to the newly created user
USER hostuser

RUN rustup component add rustfmt

# Cache dependencies to speed up build time
RUN cargo install sccache 
ENV RUSTC_WRAPPER=sccache

RUN cargo install cargo-watch
RUN cargo install sea-orm-cli@1.1.0
CMD ["cargo", "watch", "-x", "run"]
