FROM ubuntu:20.04
LABEL description="This is the build stage for Minix chain"

COPY ./minix /usr/local/bin

EXPOSE 30333 9999 8888 9615

VOLUME ["/data"]

WORKDIR /data

ENTRYPOINT ["/usr/local/bin/minix"]
