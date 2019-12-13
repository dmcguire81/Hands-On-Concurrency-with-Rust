FROM rust:1.39

COPY ./setup.sh /tmp

RUN bash /tmp/setup.sh