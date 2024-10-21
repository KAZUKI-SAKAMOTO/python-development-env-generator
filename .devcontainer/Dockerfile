FROM rust:latest

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    zsh \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install oh-my-zsh for a better zsh experience
RUN sh -c "\$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" || true

# Install Rust autocomplete and other useful tools
RUN rustup component add rustfmt clippy && \
    cargo install cargo-edit && \
    cargo install starship


# Set zsh as the default shell for root
RUN chsh -s /bin/zsh root


# Set up the workspace directory
WORKDIR /workspace

# Set up Starship prompt for zsh
RUN echo 'eval "$(starship init zsh)"' >> /root/.zshrc

# Install Homebrew
RUN /usr/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.zshrc
