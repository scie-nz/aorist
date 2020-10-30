FROM gitea/gitea:latest
ENV USER root
RUN apk --no-cache add jq
ENV USER git
COPY entrypoint.sh .
ENTRYPOINT ["./entrypoint.sh"]
