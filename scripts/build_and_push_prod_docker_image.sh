#!/bin/sh
docker build . -t sl_api_prod -f Dockerfile.prod
docker tag sl_api_prod:latest 938203255528.dkr.ecr.ap-northeast-1.amazonaws.com/ditl:latest
docker push 938203255528.dkr.ecr.ap-northeast-1.amazonaws.com/ditl:latest
