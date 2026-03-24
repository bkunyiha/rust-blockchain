#!/bin/bash
# Wait for a node to be ready before starting this node
# Usage: wait-for-node.sh <wait_service_name> <instance_number> <port> [is_webserver]

set -e

WAIT_SERVICE_NAME="${1}"
INSTANCE_NUMBER="${2}"
PORT="${3}"
IS_WEBSERVER="${4:-no}"

# Validate required parameters
if [ -z "${WAIT_SERVICE_NAME}" ] || [ -z "${INSTANCE_NUMBER}" ] || [ -z "${PORT}" ]; then
    echo "ERROR: Missing required parameters" >&2
    echo "Usage: wait-for-node.sh <wait_service_name> <instance_number> <port> [is_webserver]" >&2
    exit 1
fi

# Validate INSTANCE_NUMBER is numeric
if ! [[ "${INSTANCE_NUMBER}" =~ ^[0-9]+$ ]]; then
    echo "ERROR: INSTANCE_NUMBER must be numeric, got: '${INSTANCE_NUMBER}'" >&2
    exit 1
fi

# Calculate previous instance number
PREV_INSTANCE=$((INSTANCE_NUMBER - 1))

# CRITICAL: Ensure PREV_INSTANCE is never less than 1 (miners start at instance 1)
# This prevents miner_0 from ever being used
if [ ${PREV_INSTANCE} -lt 1 ]; then
    echo "ERROR: PREV_INSTANCE calculated as ${PREV_INSTANCE} (from INSTANCE_NUMBER=${INSTANCE_NUMBER})" >&2
    echo "ERROR: Miners start at instance 1, so PREV_INSTANCE must be >= 1" >&2
    if [ "${IS_WEBSERVER}" = "yes" ]; then
        echo "DEBUG: For webserver, forcing PREV_INSTANCE to 1 (miner_1)" >&2
        PREV_INSTANCE=1
    else
        echo "ERROR: Cannot proceed with PREV_INSTANCE < 1" >&2
        exit 1
    fi
fi

# If this is instance 1 and NOT a webserver, no need to wait
# Webservers always wait for miners (even instance 1 waits for miner_1)
if [ "${INSTANCE_NUMBER}" -eq 1 ] && [ "${IS_WEBSERVER}" != "yes" ]; then
    echo "Instance ${INSTANCE_NUMBER}: First instance (not webserver), no need to wait"
    exit 0
fi

# Determine previous node's hostname and port
# Docker Compose creates containers with names like: blockchain_miner_1, blockchain_miner_2
# For miners: iterate backwards through miner instances until we find one that exists
# For non-miners: use the service name and previous instance directly

PREV_HOSTNAME=""
PREV_PORT=""
READY=false

# Determine which node to wait for and wait for it to be ready
if [[ "${WAIT_SERVICE_NAME}" == *"miner"* ]]; then
    # For miners: iterate backwards through miner instances and wait for each one until ready
    # This handles cases where some miner instances don't exist (e.g., only miner_1 and miner_3)
    if [ "${IS_WEBSERVER}" = "yes" ]; then
        echo "Instance ${INSTANCE_NUMBER}: Searching for available miner (webserver connecting to miner)..."
    else
        echo "Instance ${INSTANCE_NUMBER}: Searching for available miner (miner connecting to previous miner)..."
    fi
    
    CHECK_INSTANCE=${PREV_INSTANCE}
    
    # Ensure we never check miner_0 (miners start at instance 1)
    if [ ${CHECK_INSTANCE} -lt 1 ]; then
        CHECK_INSTANCE=1
    fi
    
    # Combined loop: iterate backwards through miner instances and wait for each one
    # Stop at miner_1 (never check miner_0)
    # Docker Compose uses service names for DNS, so we use "miner" as the hostname
    # The port is calculated based on instance number
    while [ ${CHECK_INSTANCE} -ge 1 ]; do
        # Use service name "miner" for Docker Compose DNS resolution
        # Docker Compose automatically resolves service names to the appropriate container
        CHECK_HOSTNAME="miner"
        CHECK_PORT=$((2001 + CHECK_INSTANCE - 1))
        
        echo "  Checking miner instance ${CHECK_INSTANCE}: ${CHECK_HOSTNAME}:${CHECK_PORT}..."
        
        PREV_HOSTNAME="${CHECK_HOSTNAME}"
        PREV_PORT="${CHECK_PORT}"
        
        # Wait for this miner instance to be ready
        MAX_ATTEMPTS=60
        ATTEMPT=0
        
        while [ ${ATTEMPT} -lt ${MAX_ATTEMPTS} ]; do
            # Check if port is listening and ready
            # Try multiple methods for port checking (for compatibility across different environments)
            if command -v nc >/dev/null 2>&1; then
                # Method 1: netcat (preferred, most reliable)
                if nc -z -w 1 "${PREV_HOSTNAME}" "${PREV_PORT}" 2>/dev/null; then
                    READY=true
                    break
                fi
            elif command -v timeout >/dev/null 2>&1 && [ -c /dev/tcp ]; then
                # Method 2: timeout + /dev/tcp (works on most Linux systems)
                if timeout 1 bash -c "echo > /dev/tcp/${PREV_HOSTNAME}/${PREV_PORT}" 2>/dev/null; then
                    READY=true
                    break
                fi
            else
                # Method 3: /dev/tcp fallback (works on bash with /dev/tcp support)
                if (echo > /dev/tcp/${PREV_HOSTNAME}/${PREV_PORT}) 2>/dev/null; then
                    READY=true
                    break
                fi
            fi
            
            ATTEMPT=$((ATTEMPT + 1))
            # Log progress: first 5 attempts, then every 10th attempt
            if [ ${ATTEMPT} -le 5 ] || [ $((ATTEMPT % 10)) -eq 0 ]; then
                echo "    Attempt ${ATTEMPT}/${MAX_ATTEMPTS}: Waiting for ${PREV_HOSTNAME}:${PREV_PORT}..."
            fi
            sleep 2
        done
        
        if [ "${READY}" = "true" ]; then
            echo "  Miner ${PREV_HOSTNAME}:${PREV_PORT} is ready!"
            echo "Instance ${INSTANCE_NUMBER}: Previous node is ready!"
            # Output the connect nodes address for use by the entrypoint
            # Format: PREV_NODE_ADDRESS=hostname:port
            # Use miner_${CHECK_INSTANCE} format for the address (entrypoint will resolve it)
            OUTPUT_HOSTNAME="miner_${CHECK_INSTANCE}"
            echo "PREV_NODE_ADDRESS=${OUTPUT_HOSTNAME}:${PREV_PORT}"
            # Also export for potential use
            export PREV_NODE_ADDRESS="${OUTPUT_HOSTNAME}:${PREV_PORT}"
            exit 0
        else
            echo "  Miner ${PREV_HOSTNAME}:${PREV_PORT} did not become ready, trying next instance..."
            READY=false
            CHECK_INSTANCE=$((CHECK_INSTANCE - 1))
            # Never check miner_0 - stop at miner_1
            if [ ${CHECK_INSTANCE} -lt 1 ]; then
                echo "  ERROR: Reached miner_0 - miners start at instance 1" >&2
                break
            fi
        fi
    done
    
    # If we exit the loop without finding a ready miner, it's an error
    if [ "${READY}" != "true" ]; then
        echo "ERROR: No available miner found or ready (checked instances ${PREV_INSTANCE} down to 1)" >&2
        echo "ERROR: This usually means:" >&2
        echo "ERROR:   1. No miners are running" >&2
        echo "ERROR:   2. Miners are not listening on expected ports" >&2
        echo "ERROR:   3. Network connectivity issues" >&2
        exit 1
    fi
else
    # Error: WAIT_SERVICE_NAME should always be "miner" in normal operation
    echo "ERROR: WAIT_SERVICE_NAME is '${WAIT_SERVICE_NAME}', but expected 'miner'" >&2
    echo "  This indicates a bug in docker-entrypoint.sh or manual script invocation with wrong parameters" >&2
    echo "  Miners and webservers should always wait for miners (WAIT_SERVICE_NAME='miner')" >&2
    exit 1
fi
