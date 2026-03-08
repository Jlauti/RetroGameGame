FROM rust:1.77-bookworm
ENV PATH="/usr/local/cargo/bin:/usr/local/rustup/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
    openssh-server \
    rsync \
    git \
    pkg-config \
    cmake \
    clang \
    lld \
    ca-certificates \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libx11-dev \
    libxcursor-dev \
    libxi-dev \
    libxinerama-dev \
    libxrandr-dev \
    libvulkan-dev \
    && rm -rf /var/lib/apt/lists/*

RUN ln -sf /usr/local/cargo/bin/cargo /usr/local/bin/cargo \
    && ln -sf /usr/local/cargo/bin/rustc /usr/local/bin/rustc \
    && ln -sf /usr/local/cargo/bin/rustup /usr/local/bin/rustup

RUN mkdir -p /var/run/sshd /work /root/.ssh \
    && chmod 700 /root/.ssh \
    && sed -ri 's/^#?PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config \
    && sed -ri 's/^#?PermitRootLogin .*/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config \
    && printf "\nPubkeyAuthentication yes\nAllowUsers root\nSetEnv CARGO_HOME=/usr/local/cargo RUSTUP_HOME=/usr/local/rustup PATH=/usr/local/cargo/bin:/usr/local/rustup/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\n" >> /etc/ssh/sshd_config

COPY start-sshd.sh /usr/local/bin/start-sshd.sh
RUN chmod +x /usr/local/bin/start-sshd.sh && ssh-keygen -A

WORKDIR /work/RetroGameGame
EXPOSE 22

CMD ["/usr/local/bin/start-sshd.sh"]
