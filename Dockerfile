FROM rust:1.39-stretch

COPY ./setup.sh /tmp

RUN bash /tmp/setup.sh