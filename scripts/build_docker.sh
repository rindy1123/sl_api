#!/bin/bash

USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose build
