#!/bin/bash

# Define the compose files
COMPOSE_FILES="-f dc-properties.yml -f dc-users.yml -f dc-auth.yml -f dc-appointments.yml"

# Define the service (optional, leave blank to start all)
SERVICE=""

# Run the command
docker compose $COMPOSE_FILES up -d $SERVICE
