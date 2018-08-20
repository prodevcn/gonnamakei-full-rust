# THIS SCRIPT MUST BE EXECUTED IN THE PRODUCTION MACHINE.
# WARN docker-compose.prod.init.yaml must be copied as docker-compose.init.yaml
# WARN docker-compose.prod.yaml must be copied as docker-compose.yaml

# Load images into docker.
docker load -i gmi-api-server.gz
docker load -i gmi-scripts.gz

# Reset DB if user wants.
read -n1 -p "Reset DB? [y/N] " input
if [[ $input == "Y" || $input == "y" ]]; then
  docker-compose -f docker-compose.init.yaml up -d db
  n=1
  while [[ $n != 0 ]]
  do
    echo "Try to reset DB"
    docker-compose -f docker-compose.init.yaml up scripts

    result="$(docker ps -a | grep -i "ubuntu_scripts")"
    if [[ $result == *"Exited (1)"* ]]; then
      n=1
    else
      n=0
    fi
  done
  docker-compose -f docker-compose.init.yaml down
else
  echo "Skipping resetting DB"
fi

# Launch all services.
docker-compose up -d