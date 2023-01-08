FROM gcr.io/distroless/cc

COPY ./target/release/cli /usr/bin/cobase
COPY ./cli/migrations .
COPY ./web/dist ./static

EXPOSE 80

ENTRYPOINT ["cobase"]
CMD ["serve"]
