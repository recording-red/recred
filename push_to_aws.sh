#!/bin/bash
# see https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-push-ecr-image.html

AWS_REGION=eu-west-3
AWS_ACCOUNT_ID=123456789
DOCKER_TAG=recred

aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com

docker image rm $DOCKER_TAG
docker build . -t $DOCKER_TAG
docker tag $DOCKER_TAG:latest $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/recred:latest
docker push $AWS_ACCOUNT_ID.dkr.ecr.$AWS_REGION.amazonaws.com/$DOCKER_TAG:latest
