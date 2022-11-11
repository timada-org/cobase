FROM gcr.io/distroless/cc

COPY ./target/release/cli /usr/bin/cobase

EXPOSE 80

ENTRYPOINT ["cobase"]
CMD ["serve"]
