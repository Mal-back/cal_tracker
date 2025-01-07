all : 
	docker compose -f ./srcs/docker-compose.yml up -d --build
down :
	docker compose -f ./srcs/docker-compose.yml down -t 10

dev:
	docker compose -f ./dev_container/docker-compose.yml up -d --build

dev-down:
	docker compose -f ./dev_container/docker-compose.yml down -t 10

clean:
	docker stop $$(docker ps -qa);\
	docker rm $$(docker ps -qa);\
	docker rmi -f $$(docker images -qa);\
	docker volume rm $$(docker volume ls -q);\
	rm -rf srcs/volumes/avatar_media/users_avatars/*
