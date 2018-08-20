# Compiles the code inside a docker container to cross compile.
docker run --rm -v `pwd`/../..:/home/project -ti rust:1.55 bash -c "cd /home/project/gonnamakeit-app && ./deployment/compile_inside_docker.sh"

## Build images.
docker build ../.. -f docker/Dockerfile-api-server -t "gmi-api-server:latest"
docker build ../.. -f docker/Dockerfile-scripts -t "gmi-scripts:latest"

## Save images in a file.
docker save "gmi-api-server:latest" | gzip -c > gmi-api-server.gz
docker save "gmi-scripts:latest" | gzip -c > gmi-scripts.gz