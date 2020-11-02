FROM gitea/gitea:latest
ENV USER root
RUN apk add jq
ENV USER git
COPY entrypoint.sh .
CMD ./entrypoint.sh
