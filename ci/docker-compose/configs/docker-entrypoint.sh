#!/bin/bash
set -e

# Debug logging control (set DEBUG=1 to enable verbose logging)
DEBUG="${DEBUG:-0}"

# Helper function for debug logging
debug_log() {
    if [ "${DEBUG}" = "1" ]; then
        echo "DEBUG: $*" >&2
    fi
}

# Default values
# NODE_IS_MINER defaults to "no" (webserver mode) as a safe default:
# - If container is run directly without docker-compose, it won't accidentally start mining
# - docker-compose.yml explicitly sets NODE_IS_MINER=yes for miner service (overrides this default)
# - The entrypoint script also detects service type from container name, but this env var is what gets passed to the binary
NODE_IS_MINER="${NODE_IS_MINER:-no}"
NODE_IS_WEB_SERVER="${NODE_IS_WEB_SERVER:-yes}"

# Normalize NODE_CONNECT_NODES: handle empty string, whitespace-only, or unset
# Empty string should be treated as "local" (seed node)
if [ -z "${NODE_CONNECT_NODES}" ] || [ -z "$(echo "${NODE_CONNECT_NODES}" | xargs)" ]; then
    NODE_CONNECT_NODES="local"
else
    # Trim whitespace
    NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)
    # If trimming resulted in empty string, default to "local"
    if [ -z "${NODE_CONNECT_NODES}" ]; then
        NODE_CONNECT_NODES="local"
    fi
fi

NODE_MINING_ADDRESS="${NODE_MINING_ADDRESS:-}"

# Helper function to resolve hostname:port to IP:port
# Docker service names with underscores (e.g., miner_1) need to be resolved to IP addresses
# because Rust's SocketAddr::from_str() doesn't accept hostnames with underscores
resolve_hostname_to_ip() {
    local addr="${1}"
    local max_retries="${2:-5}"  # Default to 5 retries
    local retry_delay="${3:-2}"  # Default to 2 seconds between retries
    
    # Validate input is not empty
    if [ -z "${addr}" ]; then
        echo "ERROR: resolve_hostname_to_ip called with empty address" >&2
        return 1
    fi
    
    # Trim whitespace
    addr=$(echo "${addr}" | xargs)
    
    # Validate again after trimming
    if [ -z "${addr}" ]; then
        echo "ERROR: resolve_hostname_to_ip called with empty address (after trimming whitespace)" >&2
        return 1
    fi
    
    # If it's already an IP address or "local", return as-is
    if [[ "${addr}" == "local" ]] || [[ "${addr}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+:[0-9]+$ ]]; then
        echo "${addr}"
        return 0
    fi
    
    # Extract hostname and port
    local hostname="${addr%%:*}"
    local port="${addr##*:}"
    
    # Validate port is numeric
    if ! [[ "${port}" =~ ^[0-9]+$ ]]; then
        echo "ERROR: Invalid port '${port}' in address '${addr}'" >&2
        return 1
    fi
    
    # Try multiple methods to resolve hostname to IP with retries
    local ip=""
    local attempt=0
    
    while [ ${attempt} -lt ${max_retries} ]; do
        attempt=$((attempt + 1))
        
        # Debug: Log resolution attempt
        if [ ${attempt} -gt 1 ]; then
            debug_log "Retry ${attempt}/${max_retries}: Resolving hostname '${hostname}' to IP address..."
            sleep ${retry_delay}
        else
            debug_log "Resolving hostname '${hostname}' to IP address (attempt ${attempt}/${max_retries})..."
        fi
        
        # Method 1: Try getent hosts (preferred, works with Docker's internal DNS)
        if command -v getent >/dev/null 2>&1; then
            debug_log "Trying getent hosts ${hostname}..."
            local getent_output
            getent_output=$(getent hosts "${hostname}" 2>&1)
            local getent_exit=$?
            
            if [ ${getent_exit} -eq 0 ]; then
                ip=$(echo "${getent_output}" | awk '{print $1}' | head -n1)
                debug_log "getent result: '${ip}'"
                if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                    debug_log "Successfully resolved ${hostname} to ${ip}"
                    echo "${ip}:${port}"
                    return 0
                fi
            else
                debug_log "getent failed with exit code ${getent_exit}"
            fi
        else
            debug_log "getent not available"
        fi
        
        # Method 2: Try nslookup (if getent failed and nslookup is available)
        if [ -z "${ip}" ] && command -v nslookup >/dev/null 2>&1; then
            debug_log "Trying nslookup ${hostname}..."
            ip=$(nslookup "${hostname}" 2>/dev/null | grep -A1 "Name:" | grep "Address:" | awk '{print $2}' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via nslookup"
                echo "${ip}:${port}"
                return 0
            fi
        fi
        
        # Method 3: Try host command (if both above failed and host is available)
        if [ -z "${ip}" ] && command -v host >/dev/null 2>&1; then
            debug_log "Trying host ${hostname}..."
            ip=$(host "${hostname}" 2>/dev/null | grep "has address" | awk '{print $4}' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via host"
                echo "${ip}:${port}"
                return 0
            fi
        fi
        
        # Method 4: Try ping with timeout (as last resort, works with Docker DNS)
        if [ -z "${ip}" ] && command -v ping >/dev/null 2>&1; then
            debug_log "Trying ping ${hostname}..."
            # Use ping to resolve hostname (ping resolves DNS but we just need the IP)
            # Extract IP from ping output: "PING miner_1 (172.18.0.2)"
            ip=$(ping -c 1 -W 1 "${hostname}" 2>/dev/null | grep -oE '\([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+\)' | tr -d '()' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via ping"
                echo "${ip}:${port}"
                return 0
            fi
        fi
    done
    
    # If all resolution methods failed after retries, fail with error
    echo "ERROR: Failed to resolve hostname '${hostname}' to IP address after ${max_retries} attempts." >&2
    echo "ERROR: Tried methods: getent, nslookup, host, ping" >&2
    echo "ERROR: Docker service names should resolve via Docker's internal DNS." >&2
    echo "ERROR: This usually means:" >&2
    echo "ERROR:   1. The target service is not running yet" >&2
    echo "ERROR:   2. Docker DNS is not ready" >&2
    echo "ERROR:   3. The hostname is incorrect" >&2
    return 1
}

# Helper function to validate and fix miner_0 addresses for webservers
# Returns 0 if valid or fixed, 1 if invalid and cannot be fixed
validate_and_fix_miner_address() {
    local addr="${1}"
    local is_webserver="${2:-no}"
    
    # Check if address contains miner_0
    if [[ "${addr}" =~ miner_0 ]]; then
        if [ "${is_webserver}" = "yes" ]; then
            echo "ERROR: Invalid address '${addr}' - webservers cannot connect to miner_0" >&2
            echo "DEBUG: Overriding with miner_1:2001 for webserver" >&2
            echo "miner_1:2001"
            return 0
        else
            echo "ERROR: Cannot use miner_0 address '${addr}' - miners start at instance 1" >&2
            return 1
        fi
    fi
    
    # Extract hostname part to check for miner_0
    local hostname_only=$(echo "${addr}" | cut -d':' -f1)
    if [ "${hostname_only}" = "miner_0" ]; then
        if [ "${is_webserver}" = "yes" ]; then
            echo "ERROR: Hostname is miner_0: '${addr}'" >&2
            echo "DEBUG: Overriding with miner_1:2001 for webserver" >&2
            echo "miner_1:2001"
            return 0
        else
            echo "ERROR: Cannot use miner_0 hostname '${addr}'" >&2
            return 1
        fi
    fi
    
    # Address is valid
    echo "${addr}"
    return 0
}

# Helper function to construct PREV_ADDR for a node
construct_prev_addr() {
    local instance_number="${1}"
    local is_webserver="${2:-no}"
    local is_miner="${3:-no}"
    local wait_service_name="${4:-miner}"
    
    if [ "${is_webserver}" = "yes" ] && [ "${is_miner}" = "no" ]; then
        # Webservers always connect to miner_1
        echo "miner_1:2001"
    else
        # Miners connect to previous miner
        local prev_instance=$((instance_number - 1))
        if [ ${prev_instance} -lt 1 ]; then
            echo "ERROR: Cannot connect to miner_0 - miners start at instance 1" >&2
            return 1
        fi
        local prev_p2p_port=$((2001 + prev_instance - 1))
        local prev_hostname="${wait_service_name}_${prev_instance}"
        echo "${prev_hostname}:${prev_p2p_port}"
    fi
}

# Determine instance number from container name or environment variable
#
# Container Naming Pattern:
# Docker Compose automatically creates containers with names following this pattern:
#   <project>_<service>_<instance_number>
#
# Where:
#   - <project>: Defaults to directory name (e.g., "blockchain") or can be set via:
#     * COMPOSE_PROJECT_NAME environment variable
#     * -p/--project-name flag in docker-compose commands
#   - <service>: Service name from docker-compose.yml (e.g., "miner", "webserver")
#   - <instance_number>: Instance number when scaling (1, 2, 3, etc.)
#
# Examples:
#   - blockchain_miner_1 (project=blockchain, service=miner, instance=1)
#   - blockchain_miner_2 (project=blockchain, service=miner, instance=2)
#   - blockchain_webserver_1 (project=blockchain, service=webserver, instance=1)
#
# How HOSTNAME is Set:
# Docker automatically sets the HOSTNAME environment variable to match the container name.
# This happens in Docker's daemon code (not in this repository) when the container starts.
# The entrypoint script reads HOSTNAME to get the container name.
CONTAINER_NAME="${HOSTNAME:-}"
if [ -z "${INSTANCE_NUMBER:-}" ]; then
    # Try to extract instance number from container name
    # Pattern: <service>_<number> or <project>_<service>_<number>
    # Also supports Kubernetes StatefulSet pattern: miner-0, miner-1, etc.
    if [[ "${CONTAINER_NAME}" =~ _([0-9]+)$ ]]; then
        # Docker Compose pattern: blockchain_miner_1, blockchain_miner_2
        INSTANCE_NUMBER="${BASH_REMATCH[1]}"
    elif [[ "${CONTAINER_NAME}" =~ -([0-9]+)$ ]]; then
        # Kubernetes StatefulSet pattern: miner-0, miner-1, webserver-0, etc.
        # Extract ordinal and convert to instance number (0-based -> 1-based)
        ORDINAL="${BASH_REMATCH[1]}"
        INSTANCE_NUMBER=$((ORDINAL + 1))
    else
        # Default to 1 if we can't determine
        INSTANCE_NUMBER=1
    fi
fi

# Determine service name from container name
# Extract service name (miner or webserver) from container name
SERVICE_NAME_FROM_CONTAINER=""
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_NAME_FROM_CONTAINER="miner"
elif [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_NAME_FROM_CONTAINER="webserver"
fi

# Determine service type (miner or webserver) from container name or environment
SERVICE_TYPE=""
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_TYPE="miner"
    # Miners: P2P ports start at 2001
    P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))
elif [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_TYPE="webserver"
    # Webservers: Web ports start at 8080, P2P ports start at 2101
    WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))
    P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))
else
    # Fallback: use environment variables to determine
    if [ "${NODE_IS_MINER}" = "yes" ]; then
        SERVICE_TYPE="miner"
        P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))
    else
        SERVICE_TYPE="webserver"
        WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))
        P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))
    fi
fi

# Kubernetes mode: keep *container* ports stable.
#
# In Docker Compose we vary ports per instance on the host (and sometimes inside the container),
# but in Kubernetes each pod has its own IP, so all pods can (and should) listen on the same
# internal ports. Our manifests and probes assume:
# - miner P2P: 2001
# - webserver HTTP: 8080
# - webserver P2P: 2001
if [ -n "${POD_NAME:-}" ]; then
    if [ "${NODE_IS_MINER}" = "yes" ]; then
        P2P_PORT=2001
    fi
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
        WEB_PORT=8080
        P2P_PORT=2001
    fi
fi

# Instance-specific data directory
# Each instance gets its own isolated blockchain data directory
DATA_DIR="data${INSTANCE_NUMBER}"
BLOCKS_TREE="blocks${INSTANCE_NUMBER}"

# Base data directory (where volumes are mounted)
# Volume is mounted at /app/data, so we store instance data within it
BASE_DATA_DIR="/app/data"

# Instance-specific directory name (relative to base)
# This will be stored as /app/data/data1, /app/data/data2, etc.
INSTANCE_DATA_DIR_NAME="${DATA_DIR}"

# Full path to instance-specific data directory within the volume
INSTANCE_DATA_DIR="${BASE_DATA_DIR}/${INSTANCE_DATA_DIR_NAME}"

# Update environment variables for this instance
# TREE_DIR should be relative to current working directory (/app)
# So we use "data/data1", "data/data2", etc. to store within the mounted volume
export NODE_ADDR="0.0.0.0:${P2P_PORT}"
export TREE_DIR="data/${INSTANCE_DATA_DIR_NAME}"
export BLOCKS_TREE="${BLOCKS_TREE}"

# Create instance-specific data directory if it doesn't exist
# This ensures each node has its own isolated blockchain data within the volume
mkdir -p "${INSTANCE_DATA_DIR}"
echo "Using isolated blockchain data directory: ${INSTANCE_DATA_DIR}"
echo "  TREE_DIR=${TREE_DIR} (relative to /app)"
echo "  BLOCKS_TREE=${BLOCKS_TREE}"

# Auto-configure webservers to connect to first miner if NODE_CONNECT_NODES is "local" or empty
# This ensures webservers connect to miners, not act as seed nodes
# NOTE: For webserver instance 1, we set it to "local" here and let sequential startup handle
# the resolution after waiting for the miner to be ready. This ensures the miner is available
# before we try to resolve its hostname.
if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
    # Normalize empty string to "local" for consistency
    # For webservers, "local" means "wait for miner and connect to it" (handled by sequential startup)
    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ] || [ "${NODE_CONNECT_NODES}" = "" ]; then
        # Set to "local" - sequential startup will handle waiting for miner and resolution
        NODE_CONNECT_NODES="local"
        echo "Webserver instance ${INSTANCE_NUMBER}: Will connect to miner (sequential startup will handle resolution)"
    else
        # Trim whitespace and validate it's not empty after trimming
        NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)
        if [ -z "${NODE_CONNECT_NODES}" ]; then
            echo "WARNING: NODE_CONNECT_NODES is empty after trimming whitespace" >&2
            echo "WARNING: Setting to 'local' - sequential startup will handle resolution" >&2
            NODE_CONNECT_NODES="local"
        else
            # If it's already an IP address, keep it as-is
            if [[ "${NODE_CONNECT_NODES}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+:[0-9]+$ ]]; then
                echo "Webserver instance ${INSTANCE_NUMBER}: Using configured IP address: ${NODE_CONNECT_NODES}"
            else
                # Contains a hostname - sequential startup will wait for service and resolve it
                echo "Webserver instance ${INSTANCE_NUMBER}: Will resolve hostname '${NODE_CONNECT_NODES}' after waiting for service"
                # Don't resolve yet - let sequential startup handle it after waiting
            fi
        fi
    fi
fi

# Kubernetes StatefulSet Support: Auto-configure NODE_CONNECT_NODES for miners
# StatefulSet pods have stable names: miner-0, miner-1, miner-2, etc.
# Pods can connect using headless service DNS: miner-0.miner-headless.blockchain.svc.cluster.local
if [ "${NODE_IS_MINER}" = "yes" ] && [ -n "${POD_NAME}" ]; then
    # Extract ordinal from pod name (e.g., miner-0 -> 0, miner-1 -> 1)
    if [[ "${POD_NAME}" =~ -([0-9]+)$ ]]; then
        ORDINAL="${BASH_REMATCH[1]}"
        
        if [ "${ORDINAL}" = "0" ]; then
            # First miner (miner-0) starts as seed node
            if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                NODE_CONNECT_NODES="local"
                echo "Kubernetes StatefulSet: First miner (miner-0) starting as seed node"
            fi
        else
            # Subsequent miners connect to previous miner via headless service
            PREV_ORDINAL=$((ORDINAL - 1))
            if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                # Use Kubernetes DNS format for headless service
                NODE_CONNECT_NODES="miner-${PREV_ORDINAL}.miner-headless.blockchain.svc.cluster.local:2001"
                echo "Kubernetes StatefulSet: Miner ${ORDINAL} connecting to miner-${PREV_ORDINAL}"
            fi
        fi
    fi
fi

# Sequential startup: Wait for previous node if enabled (Docker Compose mode)
# For webservers, we always wait for miners (even instance 1) to ensure miner is ready
# For miners, only wait if instance number > 1
if [ "${SEQUENTIAL_STARTUP:-yes}" = "yes" ]; then
    # Webservers always wait for miners (even instance 1)
    # Miners only wait if not the first instance
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
        # All webservers wait for miners
        SHOULD_WAIT=true
    elif [ "${NODE_IS_MINER}" = "yes" ] && [ "${INSTANCE_NUMBER}" -gt 1 ]; then
        # Miners wait only if not first instance
        SHOULD_WAIT=true
    else
        SHOULD_WAIT=false
    fi
    
    if [ "${SHOULD_WAIT}" = "true" ]; then
        # Skip sequential startup logic if we're in Kubernetes StatefulSet mode
        # (StatefulSet handles ordered startup automatically)
        if [ -z "${POD_NAME}" ] || [[ ! "${POD_NAME}" =~ -([0-9]+)$ ]]; then
            if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
                echo "Sequential startup enabled: Webserver waiting for miner to be ready..."
            else
                echo "Sequential startup enabled: Waiting for previous node..."
            fi
        
        # Determine service name for wait script
        # Miners connect to previous miner
        # Webservers ALWAYS connect to miners (never to other webservers)
        WAIT_SERVICE_NAME="miner"
        
        # Run wait script and capture output
        if [ -f "/app/wait-for-node.sh" ]; then
            # For webservers, we need to pass a higher instance number so the wait script
            # looks for miner_1 (not miner_0). The wait script calculates PREV_INSTANCE = INSTANCE_NUMBER - 1
            # So for webserver instance 1, pass instance 2 to get PREV_INSTANCE = 1 (miner_1)
            WAIT_INSTANCE_NUMBER="${INSTANCE_NUMBER}"
            if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
                # Webservers always wait for miners, so use instance number that will result in miner_1
                # If webserver is instance 1, pass 2 to get miner_1 (2-1=1)
                WAIT_INSTANCE_NUMBER=$((INSTANCE_NUMBER + 1))
            fi
            debug_log "Calling wait script with:"
            debug_log "  WAIT_SERVICE_NAME=${WAIT_SERVICE_NAME}"
            debug_log "  WAIT_INSTANCE_NUMBER=${WAIT_INSTANCE_NUMBER}"
            debug_log "  INSTANCE_NUMBER=${INSTANCE_NUMBER}"
            debug_log "  NODE_IS_WEB_SERVER=${NODE_IS_WEB_SERVER}"
            WAIT_OUTPUT=$(/app/wait-for-node.sh "${WAIT_SERVICE_NAME}" "${WAIT_INSTANCE_NUMBER}" "${P2P_PORT}" "${NODE_IS_WEB_SERVER}" 2>&1)
            WAIT_EXIT_CODE=$?
            debug_log "Wait script exit code: ${WAIT_EXIT_CODE}"
            
            # Display wait script output
            echo "${WAIT_OUTPUT}"
            
            if [ ${WAIT_EXIT_CODE} -eq 0 ]; then
                # Extract PREV_NODE_ADDRESS from output if present
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                debug_log "Extracted PREV_ADDR from wait script: '${PREV_ADDR}'"
                
                # Construct previous node address if not provided
                if [ -z "${PREV_ADDR}" ]; then
                    debug_log "PREV_ADDR not found in wait script output, constructing..."
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "${WAIT_SERVICE_NAME}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    debug_log "Constructed PREV_ADDR: ${PREV_ADDR}"
                else
                    # Validate extracted address - webservers should never get miner_0
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "${NODE_IS_WEB_SERVER}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                
                debug_log "Final PREV_ADDR before resolution: '${PREV_ADDR}'"
                
                # Resolve Docker service name to IP address
                # Docker service names with underscores (e.g., miner_1) need to be resolved to IP
                # For Docker Compose, "miner_1" doesn't resolve, but "miner" does
                # So we convert miner_1 to miner for resolution purposes
                RESOLVE_ADDR="${PREV_ADDR}"
                if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
                    # Extract instance number and use "miner" service name for resolution
                    INSTANCE_NUM="${BASH_REMATCH[1]}"
                    PORT_PART="${PREV_ADDR##*:}"
                    RESOLVE_ADDR="miner:${PORT_PART}"
                    debug_log "Converting ${PREV_ADDR} to ${RESOLVE_ADDR} for Docker Compose DNS resolution"
                fi
                
                if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
                    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}' (from '${PREV_ADDR}')" >&2
                    exit 1
                fi
                
                # Use the previous node address for NODE_CONNECT_NODES
                # Miners: connect to previous miner
                # Webservers: always connect to miners (first miner for all webservers)
                if [ "${NODE_IS_MINER}" = "yes" ]; then
                    # Miners connect to previous miner
                    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                        NODE_CONNECT_NODES="${PREV_ADDR_RESOLVED}"
                        echo "  Auto-configured connect nodes: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                    else
                        # Resolve configured connect nodes if they contain hostnames
                        if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                            echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}'" >&2
                            exit 1
                        fi
                        echo "  Using configured connect nodes: ${NODE_CONNECT_NODES}"
                    fi
                else
                    # Webservers always connect to first miner (miner_1:2001)
                    # Sequential startup ensures miner_1 is ready before webservers start
                    # Use "miner:2001" for Docker Compose DNS resolution
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner: miner:2001 -> ${NODE_CONNECT_NODES}"
                fi
            else
                echo "  ERROR: Wait script failed (exit code ${WAIT_EXIT_CODE}), but continuing startup..." >&2
                # Even if wait script failed, try to extract PREV_NODE_ADDRESS from output
                # (in case it outputted the address before failing)
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                debug_log "Extracted PREV_ADDR from failed wait script: '${PREV_ADDR}'"
                
                # If we didn't get an address from wait script, construct it
                if [ -z "${PREV_ADDR}" ]; then
                    debug_log "Constructing PREV_ADDR after wait script failure..."
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "${WAIT_SERVICE_NAME}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    debug_log "Constructed PREV_ADDR: ${PREV_ADDR}"
                else
                    # Validate extracted address - webservers should never get miner_0
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "${NODE_IS_WEB_SERVER}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                
                debug_log "PREV_ADDR after wait script failure handling: '${PREV_ADDR}'"
                
                # Resolve Docker service name to IP address
                # For Docker Compose, "miner_1" doesn't resolve, but "miner" does
                RESOLVE_ADDR="${PREV_ADDR}"
                if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
                    INSTANCE_NUM="${BASH_REMATCH[1]}"
                    PORT_PART="${PREV_ADDR##*:}"
                    RESOLVE_ADDR="miner:${PORT_PART}"
                    debug_log "Converting ${PREV_ADDR} to ${RESOLVE_ADDR} for Docker Compose DNS resolution"
                fi
                
                if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
                    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}' (from '${PREV_ADDR}')" >&2
                    exit 1
                fi
                
                # Use the previous node address for NODE_CONNECT_NODES
                if [ "${NODE_IS_MINER}" = "yes" ]; then
                    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                        NODE_CONNECT_NODES="${PREV_ADDR_RESOLVED}"
                        echo "  Auto-configured connect nodes: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                    fi
                else
                    # Webservers always connect to first miner (miner_1:2001)
                    # Use "miner:2001" for Docker Compose DNS resolution
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner: miner:2001 -> ${NODE_CONNECT_NODES}"
                fi
            fi
        else
            echo "  Warning: wait-for-node.sh not found, skipping wait"
            # Still construct node address even without wait script
            if [ "${NODE_IS_MINER}" = "yes" ]; then
                # Miners connect to previous miner
                if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "miner")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${PREV_ADDR}"); then
                        echo "ERROR: Failed to resolve previous miner address '${PREV_ADDR}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Auto-configured connect nodes (no wait): ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                else
                    # Resolve configured connect nodes if they contain hostnames
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                        echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Using configured connect nodes (no wait): ${NODE_CONNECT_NODES}"
                fi
            else
                # Webservers always connect to first miner (miner_1, not miner_0)
                # Use "miner:2001" for Docker Compose DNS resolution
                if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver (no wait)" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner (no wait): miner:2001 -> ${NODE_CONNECT_NODES}"
                else
                    # Resolve configured connect nodes if they contain hostnames
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                        echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Using configured connect nodes (no wait): ${NODE_CONNECT_NODES}"
                fi
            fi
        fi
        fi  # Close POD_NAME check
    fi  # Close SHOULD_WAIT check
else
    # Sequential startup disabled - for webservers, still try to wait for miner if instance 1
    # IMPORTANT: This Docker-Compose-style wait logic should NOT run in Kubernetes.
    # In Kubernetes, we use initContainers (and/or readiness probes) for ordering and service availability.
    if [ -n "${POD_NAME:-}" ]; then
        debug_log "Kubernetes mode detected (POD_NAME set) - skipping Docker Compose sequential wait logic"
    else
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ] && [ "${INSTANCE_NUMBER}" -eq 1 ]; then
        echo "Sequential startup disabled, but webserver instance 1 will wait for miner..."
        if [ -f "/app/wait-for-node.sh" ]; then
            # For webservers, we always wait for miner_1, not a previous webserver instance
            # Pass instance number 2 so wait script looks for miner_1 (2-1=1)
            WAIT_OUTPUT=$(/app/wait-for-node.sh "miner" "2" "${P2P_PORT}" "yes" 2>&1)
            WAIT_EXIT_CODE=$?
            echo "${WAIT_OUTPUT}"
            if [ ${WAIT_EXIT_CODE} -eq 0 ]; then
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                if [ -z "${PREV_ADDR}" ]; then
                    PREV_ADDR="miner_1:2001"
                else
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "yes")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                if [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${PREV_ADDR}"); then
                        echo "ERROR: Failed to resolve miner address '${PREV_ADDR}' for webserver instance 1" >&2
                        exit 1
                    fi
                    echo "Webserver instance 1: Resolved miner address: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                fi
            else
                echo "WARNING: Wait script failed, but continuing..." >&2
            fi
        fi
    fi
    fi
fi

# Determine wallet address from pool or direct assignment
# Option 1: Use WALLET_ADDRESS_POOL (comma-separated list) - each instance picks by index
# Option 2: Use NODE_MINING_ADDRESS directly

# Debug: Log received environment variables
debug_log "WALLET_ADDRESS_POOL='${WALLET_ADDRESS_POOL}'"
debug_log "NODE_MINING_ADDRESS='${NODE_MINING_ADDRESS}'"
debug_log "INSTANCE_NUMBER=${INSTANCE_NUMBER}"

if [ -n "${WALLET_ADDRESS_POOL}" ]; then
    # Convert comma-separated list to array
    IFS=',' read -ra ADDRESSES <<< "${WALLET_ADDRESS_POOL}"
    # Select address based on instance number (1-indexed, so subtract 1)
    INDEX=$((INSTANCE_NUMBER - 1))
    if [ ${INDEX} -lt ${#ADDRESSES[@]} ]; then
        NODE_MINING_ADDRESS="${ADDRESSES[${INDEX}]}"
        echo "Selected address from pool (instance ${INSTANCE_NUMBER}, index ${INDEX}): ${NODE_MINING_ADDRESS}"
    else
        echo "ERROR: Not enough addresses in pool for instance ${INSTANCE_NUMBER}"
        echo "  Pool size: ${#ADDRESSES[@]}, Required index: ${INDEX}"
        echo "  Available addresses: ${WALLET_ADDRESS_POOL}"
        exit 1
    fi
fi

# Auto-generate a mining address if none is provided (Kubernetes-friendly).
#
# Why: `startnode` requires a valid wallet address argument. In Kubernetes we prefer not to
# force users to pre-generate and inject addresses. Instead, if no address is provided, we
# create one in the container and persist it to the wallet volume.
#
# We also treat the common placeholder "your-wallet-address-here" as "unset".
PLACEHOLDER_MINING_ADDR="your-wallet-address-here"
MINING_ADDR_FILE=""

# Determine wallet directory from WALLET_FILE (default: wallets.dat in /app).
WALLET_FILE_PATH="${WALLET_FILE:-wallets.dat}"
WALLET_DIR="/app/$(dirname "${WALLET_FILE_PATH}")"
if [ "${WALLET_DIR}" = "/app/." ]; then
    WALLET_DIR="/app"
fi
MINING_ADDR_FILE="${WALLET_DIR}/mining_address.txt"

if [ "${NODE_MINING_ADDRESS}" = "${PLACEHOLDER_MINING_ADDR}" ]; then
    NODE_MINING_ADDRESS=""
fi

if [ -z "${NODE_MINING_ADDRESS}" ]; then
    # Prefer previously persisted mining address (stable across restarts).
    if [ -f "${MINING_ADDR_FILE}" ]; then
        NODE_MINING_ADDRESS="$(cat "${MINING_ADDR_FILE}" | head -n1 | xargs || true)"
    fi
fi

if [ -z "${NODE_MINING_ADDRESS}" ]; then
    echo "NODE_MINING_ADDRESS not provided; creating a new wallet address for mining..."
    echo "  Wallet file: /app/${WALLET_FILE_PATH}"
    echo "  Mining address cache: ${MINING_ADDR_FILE}"

    CREATE_OUTPUT=$(/app/blockchain createwallet 2>&1 || true)
    # The CLI prints: "Your new address: <ADDR>"
    NEW_ADDR=$(echo "${CREATE_OUTPUT}" | sed -n 's/.*Your new address: \([^[:space:]]\+\).*/\1/p' | tail -n1)

    if [ -z "${NEW_ADDR}" ]; then
        echo "ERROR: Failed to create wallet address. Raw output:" >&2
        echo "${CREATE_OUTPUT}" >&2
        exit 1
    fi

    NODE_MINING_ADDRESS="${NEW_ADDR}"
    mkdir -p "${WALLET_DIR}"
    echo "${NODE_MINING_ADDRESS}" > "${MINING_ADDR_FILE}"
    echo "Generated mining address: ${NODE_MINING_ADDRESS}"
fi

# Final resolution: Ensure NODE_CONNECT_NODES doesn't contain hostnames with underscores
# This handles cases where NODE_CONNECT_NODES was set via environment variables (e.g., docker-compose.yml)
# and might contain Docker service names like "miner_1:2001"
# This is a critical safety check - Rust cannot parse hostnames, only IP addresses

# Normalize empty or whitespace-only values
if [ -z "${NODE_CONNECT_NODES}" ] || [ -z "$(echo "${NODE_CONNECT_NODES}" | xargs)" ]; then
    echo "WARNING: NODE_CONNECT_NODES is empty or whitespace-only, defaulting to 'local'" >&2
    NODE_CONNECT_NODES="local"
fi

# Trim whitespace
NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)

# Validate it's still not empty after trimming
if [ -z "${NODE_CONNECT_NODES}" ]; then
    echo "ERROR: NODE_CONNECT_NODES is empty after normalization" >&2
    echo "ERROR: Cannot proceed without a valid connect nodes value" >&2
    exit 1
fi

if [ "${NODE_CONNECT_NODES}" != "local" ]; then
    # Check if it's already a valid IP:port format
    if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+:[0-9]+$ ]]; then
        # Contains a hostname (not "local" and not an IP address), resolve it
        echo "Final resolution: Resolving NODE_CONNECT_NODES '${NODE_CONNECT_NODES}' to IP address..."
        echo "  WARNING: Hostname detected - must resolve to IP before passing to Rust binary" >&2
        
        if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
            echo "ERROR: Final resolution failed for NODE_CONNECT_NODES '${NODE_CONNECT_NODES}'" >&2
            echo "ERROR: Cannot proceed - Rust binary requires IP address, not hostname" >&2
            exit 1
        fi
        
        # Verify the result is actually an IP address
        if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+:[0-9]+$ ]]; then
            echo "ERROR: Resolution returned invalid format: '${NODE_CONNECT_NODES}'" >&2
            echo "ERROR: Expected IP:port format (e.g., 172.18.0.2:2001)" >&2
            exit 1
        fi
        
        echo "Final resolution result: ${NODE_CONNECT_NODES}"
    else
        echo "Final resolution: NODE_CONNECT_NODES is already an IP address: ${NODE_CONNECT_NODES}"
    fi
else
    echo "Final resolution: NODE_CONNECT_NODES is 'local' (seed node mode)"
fi

# Final validation: Double-check that NODE_CONNECT_NODES is valid before building command
# This is a critical safety check to prevent Rust parse errors
if [ "${NODE_CONNECT_NODES}" != "local" ]; then
    if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+:[0-9]+$ ]]; then
        echo "ERROR: CRITICAL - NODE_CONNECT_NODES contains invalid value: '${NODE_CONNECT_NODES}'" >&2
        echo "ERROR: Expected 'local' or IP:port format (e.g., 172.18.0.2:2001)" >&2
        echo "ERROR: Hostnames like 'miner_1:2001' cannot be passed to Rust binary" >&2
        echo "ERROR: This indicates a bug in the entrypoint script - resolution should have occurred earlier" >&2
        exit 1
    fi
fi

# Build the command
# Format: startnode <is_miner> <is_web_server> <connect_nodes> -- <mining_address>
CMD="./blockchain startnode ${NODE_IS_MINER} ${NODE_IS_WEB_SERVER} ${NODE_CONNECT_NODES} -- ${NODE_MINING_ADDRESS}"

# Debug: Show the exact command that will be executed
debug_log "Command to execute: ${CMD}"

# Log instance configuration
echo "=========================================="
echo "Starting blockchain node"
echo "  Service Type: ${SERVICE_TYPE}"
echo "  Instance Number: ${INSTANCE_NUMBER}"
echo "  Container Name: ${CONTAINER_NAME}"
echo "  Mode: miner=${NODE_IS_MINER}, webserver=${NODE_IS_WEB_SERVER}"
echo "  P2P Port: ${P2P_PORT}"
if [ "${NODE_IS_WEB_SERVER}" = "yes" ]; then
    echo "  Web Port: ${WEB_PORT}"
fi
echo "  Data Directory: ${INSTANCE_DATA_DIR} (isolated per instance)"
echo "  TREE_DIR: ${TREE_DIR}"
echo "  Connect Nodes: ${NODE_CONNECT_NODES}"
echo "  Mining Address: ${NODE_MINING_ADDRESS}"
echo "=========================================="

# Execute the command
exec $CMD
