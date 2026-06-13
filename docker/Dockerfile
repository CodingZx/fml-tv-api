FROM debian:13.4-slim
VOLUME /tmp
ENV TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ADD ./fml-tv-api fml-tv-api
RUN sh -c 'touch /fml-tv-api'

ENV RUST_LOG="info"
ENTRYPOINT [ "sh", "-c", "exec /fml-tv-api" ]