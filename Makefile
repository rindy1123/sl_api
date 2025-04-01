USER_ID=$(shell id -u)
GROUP_ID=$(shell id -g)

d_build:
	USER_ID=$(USER_ID) GROUP_ID=$(GROUP_ID) docker compose build
