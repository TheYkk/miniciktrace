docker run --rm -d -p6831:6831/udp -p14268:14268 -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true --name jaeger jaegertracing/all-in-one:latest
