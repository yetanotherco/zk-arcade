#!/bin/bash

source .env

# Add new environment variables here
env_vars=(
  "ENVIRONMENT"
  "RPC_URL"
  "PHX_HOST"
  "DB_NAME"
  "DB_USER"
  "DB_PASS"
  "DB_HOST"
  "ALIGNED_CONFIG_FILE"
  "DEBUG_ERRORS"
  "TRACKER_API_URL"
  "MAX_BATCH_SIZE"
  "BATCH_TTL_MINUTES"
  "SCHEDULED_BATCH_INTERVAL_MINUTES"
  "LATEST_RELEASE"
  "ALIGNED_PROOF_AGG_CONFIG_FILE"
  "BEACON_CLIENT"
)

for var in "${env_vars[@]}"; do
  export "$var=${!var}"
done

mix compile --force #force recompile to get the latest .env values

elixir --sname zkarcade -S mix phx.server
