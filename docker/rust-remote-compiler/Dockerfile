FROM rust:1.77-bookworm

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

RUN mkdir -p /var/run/sshd /work /root/.ssh \
    && chmod 700 /root/.ssh \
    && sed -ri 's/^#?PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config \
    && sed -ri 's/^#?PermitRootLogin .*/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config \
    && printf "\nPubkeyAuthentication yes\nAllowUsers root\n" >> /etc/ssh/sshd_config

COPY start-sshd.sh /usr/local/bin/start-sshd.sh
RUN chmod +x /usr/local/bin/start-sshd.sh && ssh-keygen -A

WORKDIR /work/RetroGameGame
EXPOSE 22

CMD ["/usr/local/bin/start-sshd.sh"]
