DOCKER_NAME ?= arceos
CONTAINER_NAME ?= arceos_container
.PHONY: docker build_docker new_docker restart stop remove

docker:
	docker exec -it ${CONTAINER_NAME} bash

stop:
	docker stop ${CONTAINER_NAME}

restart:
	docker restart ${CONTAINER_NAME}

remove:
	docker rm -f ${CONTAINER_NAME}

new_docker:
	docker run -itd -v ${PWD}:/mnt -w /mnt --name ${CONTAINER_NAME} ${DOCKER_NAME} bash

build_docker: 
	docker build -t ${DOCKER_NAME} .
