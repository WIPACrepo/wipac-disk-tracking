#!/usr/bin/env bash
# run the testing client

export BASE_URL=${HEALTH_URL:="http://localhost:8080"}
export CLIENT_ID=${CLIENT_ID:="long-term-archive"}  # TODO: CHANGE THIS TO disk-tracking !!!
export CLIENT_SECRET=${CLIENT_SECRET:="$(<keycloak-client-secret)"}
export OAUTH_URL=${OAUTH_URL:="https://keycloak.icecube.wisc.edu/auth/realms/IceCube"}
export REST_URL=${REST_URL:="http://localhost:8080/api/v1"}

python3 client.py
