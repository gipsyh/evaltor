FROM ubuntu:24.10

RUN apt-get update && \
    apt-get install -y python3 python3-setuptools libgmp-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY evaltor /usr/local/bin
ENTRYPOINT ["evaltor"]
