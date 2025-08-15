FROM node:24

ARG TZ
ENV TZ="$TZ"

# Install basic development tools and iptables/ipset
RUN apt update && apt install -y less \
  git \
  procps \
  sudo \
  fzf \
  zsh \
  man-db \
  unzip \
  gnupg2 \
  gh \
  iptables \
  ipset \
  iproute2 \
  dnsutils \
  aggregate \
  jq \
  curl \
  build-essential \
  pkg-config \
  libssl-dev \
  sqlite3

# Ensure default node user has access to /usr/local/share
RUN mkdir -p /usr/local/share/npm-global && \
  chown -R node:node /usr/local/share

ARG USERNAME=node

# Persist bash history.
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
  && mkdir /commandhistory \
  && touch /commandhistory/.bash_history \
  && chown -R $USERNAME /commandhistory

# Set `DEVCONTAINER` environment variable to help with orientation
ENV DEVCONTAINER=true

# Install Rust toolchain as root
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default && \
    rustup component add rustfmt clippy rust-analyzer && \
    chmod -R a+w /usr/local/rustup /usr/local/cargo

# Install Rust development tools
<<<<<<< HEAD
# Install essential tools first (sqlx-cli is critical for the project)
RUN cargo install sqlx-cli

# Install optional development tools (failures won't break the build)
# These tools may have compilation issues on certain architectures
RUN cargo install cargo-watch || echo "Warning: cargo-watch installation failed (optional tool)"
RUN cargo install cargo-edit || echo "Warning: cargo-edit installation failed (optional tool)"
RUN cargo install cargo-tarpaulin || echo "Warning: cargo-tarpaulin installation failed (optional tool)"
RUN cargo install cargo-deny || echo "Warning: cargo-deny installation failed (optional tool)"
=======
RUN cargo install cargo-watch cargo-edit cargo-tarpaulin cargo-deny sqlx-cli
>>>>>>> e72c29e (WIP)

# Create workspace and config directories and set permissions
RUN mkdir -p /workspace /home/node/.claude && \
  chown -R node:node /workspace /home/node/.claude

WORKDIR /workspace

RUN wget https://github.com/dandavison/delta/releases/download/0.18.2/git-delta_0.18.2_arm64.deb && \
  sudo dpkg -i git-delta_0.18.2_arm64.deb && \
  rm git-delta_0.18.2_arm64.deb

# Set up non-root user
USER node

# Install global packages
ENV NPM_CONFIG_PREFIX=/usr/local/share/npm-global
ENV PATH=$PATH:/usr/local/share/npm-global/bin

# Set the default shell to bash rather than sh
ENV SHELL=/bin/zsh

# Default powerline10k theme
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.2.0/zsh-in-docker.sh)" -- \
  -p git \
  -p fzf \
  -a "source /usr/share/doc/fzf/examples/key-bindings.zsh" \
  -a "source /usr/share/doc/fzf/examples/completion.zsh" \
  -a "export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
  -x

# Install Claude
RUN npm install -g @anthropic-ai/claude-code

# Copy and set up firewall script
COPY init-firewall.sh /usr/local/bin/
USER root
RUN chmod +x /usr/local/bin/init-firewall.sh && \
  echo "node ALL=(root) NOPASSWD: /usr/local/bin/init-firewall.sh" > /etc/sudoers.d/node-firewall && \
  chmod 0440 /etc/sudoers.d/node-firewall
USER node
