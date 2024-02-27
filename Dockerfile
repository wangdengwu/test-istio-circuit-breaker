FROM golang:alpine as builder

WORKDIR /go/src/sayweee.com/caller
COPY . .
RUN go env -w GO111MODULE=on \
    && go env -w CGO_ENABLED=0 \
    && go build -o caller .

FROM alpine:latest

WORKDIR /go/src/sayweee.com/caller

COPY --from=0 /go/src/sayweee.com/caller/caller ./

EXPOSE 8080
ENTRYPOINT ./caller
