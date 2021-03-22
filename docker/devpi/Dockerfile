# adapted from https://github.com/ocf/devpi/blob/master/Dockerfile
# (under Apache 2 license)
FROM debian:sid
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    python3-dev \
    python3-venv \
    python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
ADD devpi.conf /
ADD requirements.txt /
RUN python3 -m pip install -r /requirements.txt
VOLUME /mnt
EXPOSE 3141
CMD ["devpi-server", "--init", "-c", "devpi.conf"]
