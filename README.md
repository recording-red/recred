# Recred

`recred` is the application backend behind recording.rec.
It uses `actix` as web framework, `sea-orm` as an async ORM engine and `postgres` as database.


## Quickstart

1. Run the following query to start the `postgres` docker image. It also spawns an instance of
   `adminer`, a web client to connect to the database:
   `docker-compose up -d`

2. Run the migrations with the following command:
   `cargo run -p migration -- up`

3. Starts the backend:
   `cargo run`


## Available endpoints

| route | method | endpoint | purpose |
| ----- | ------ | -------- | ------- |
| health | GET | /health | tests whether API is up |
| registration | GET | /registration | returns list of registrations |
| registration | POST | /registration | registers a user |
| user | GET | /user | returns list of users |
| user | GET | /user/{id} | returns a single user |
| user | POST | /user | creates user |
| language | GET | /language | returns list of languages |
| language | POST | /language | create language |
| team | GET | /team | returns list of teams |
| team | GET | /team/{id} | returns a single team |
| team | POST | /team | creates a team |
| channel | POST | /channel | create a channel |
| channel | PATCH | /channel/{id} | modifies an existing channel |
| channel | PATCH | /channel/{id}/banner | modifies channel's banner |
| channel | PATCH | /channel/{id}/profile | modifies channel's profile pic |
| channel | GET | /channel | returns list of channels |
| channel | GET | /channel/{id} | returns a single channel |
| style | POST | /style | creates a style |
| style | GET | /style | returns a list of styles |
| instrument | GET | /instrument | returns a list of instruments |
| video | POST | /video/upload | uploads a video |


## DevOp

## Terraform File
Follows these instructions: [Using Terraform to Create EC2 and RDS Instances Inside a Custom VPC on AWS](https://medium.com/strategio/using-terraform-to-create-aws-vpc-ec2-and-rds-instances-c7f3aa416133)
