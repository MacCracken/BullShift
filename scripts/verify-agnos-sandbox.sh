#!/bin/bash
# BullShift AGNOS Sandbox Verification Script
# Run this INSIDE an AGNOS sandbox environment to verify compatibility.
#
# Prerequisites:
#   - AGNOS installed with Landlock/seccomp sandbox active
#   - BullShift installed via agpkg or marketplace
#   - AGNOS_AGENT_REGISTRY_URL set (daimon must be running)
#
# Usage: ./verify-agnos-sandbox.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0
WARN=0

pass() { echo -e "  ${GREEN}PASS${NC} $1"; ((PASS++)); }
fail() { echo -e "  ${RED}FAIL${NC} $1"; ((FAIL++)); }
warn() { echo -e "  ${YELLOW}WARN${NC} $1"; ((WARN++)); }

echo "============================================"
echo "BullShift AGNOS Sandbox Verification"
echo "============================================"
echo ""

# 1. Check AGNOS environment
echo "[1/6] AGNOS Environment"
if [ -n "$AGNOS_RUNTIME_URL" ] || [ -d "/run/agnos" ]; then
    pass "Running inside AGNOS environment"
else
    warn "AGNOS runtime not detected (AGNOS_RUNTIME_URL unset, /run/agnos missing)"
fi

if id agnos &>/dev/null; then
    pass "agnos user exists"
else
    warn "agnos user not found — may be running outside AGNOS"
fi

# 2. Check binary accessibility
echo ""
echo "[2/6] Binary Accessibility"
if command -v bullshift-api &>/dev/null || [ -x /usr/local/bin/api_server ]; then
    pass "API server binary found"
else
    fail "API server binary not found in PATH or /usr/local/bin/"
fi

if [ -d "/usr/share/icons" ]; then
    if ls /usr/share/icons/bullshift* &>/dev/null; then
        pass "App icons installed in /usr/share/icons/"
    else
        fail "App icons missing from /usr/share/icons/"
    fi
else
    warn "/usr/share/icons/ directory not present"
fi

# 3. Data directory persistence
echo ""
echo "[3/6] Data Directory Persistence"
DATA_DIR="${BULLSHIFT_DATA_DIR:-$HOME/.local/share/bullshift}"
if mkdir -p "$DATA_DIR" 2>/dev/null; then
    TEST_FILE="$DATA_DIR/.sandbox-verify-$$"
    echo "test" > "$TEST_FILE" 2>/dev/null && rm "$TEST_FILE" && \
        pass "Data directory writable: $DATA_DIR" || \
        fail "Data directory not writable: $DATA_DIR"
else
    fail "Cannot create data directory: $DATA_DIR"
fi

# 4. Network: broker API connectivity
echo ""
echo "[4/6] Broker API Connectivity (sandbox allowed hosts)"
BROKER_HOSTS=(
    "api.alpaca.markets:443"
    "api.tradier.com:443"
    "api.robinhood.com:443"
    "api.schwabapi.com:443"
    "api.coinbase.com:443"
    "api.kraken.com:443"
    "api.webull.com:443"
)
for host in "${BROKER_HOSTS[@]}"; do
    if timeout 5 bash -c "echo >/dev/tcp/${host%:*}/${host#*:}" 2>/dev/null; then
        pass "Reachable: $host"
    else
        warn "Unreachable: $host (may be sandbox-blocked or DNS issue)"
    fi
done

# 5. AGNOS integration endpoints
echo ""
echo "[5/6] AGNOS Integration Endpoints"
if [ -n "$AGNOS_AGENT_REGISTRY_URL" ]; then
    if curl -sf --max-time 5 "$AGNOS_AGENT_REGISTRY_URL/health" >/dev/null 2>&1; then
        pass "Daimon reachable at $AGNOS_AGENT_REGISTRY_URL"
    else
        fail "Daimon unreachable at $AGNOS_AGENT_REGISTRY_URL"
    fi
else
    warn "AGNOS_AGENT_REGISTRY_URL not set — agent registration will be skipped"
fi

if [ -n "$AGNOS_AUDIT_URL" ]; then
    pass "Audit forwarding configured: $AGNOS_AUDIT_URL"
else
    warn "AGNOS_AUDIT_URL not set — audit forwarding disabled"
fi

if [ -n "$AGNOS_LLM_GATEWAY_URL" ]; then
    pass "LLM gateway configured: $AGNOS_LLM_GATEWAY_URL"
else
    warn "AGNOS_LLM_GATEWAY_URL not set — using direct AI provider connections"
fi

# 6. API server smoke test
echo ""
echo "[6/6] API Server Smoke Test"
BULLSHIFT_PORT="${BULLSHIFT_PORT:-8787}"
if curl -sf --max-time 5 "http://localhost:$BULLSHIFT_PORT/health" >/dev/null 2>&1; then
    pass "API server responding on port $BULLSHIFT_PORT"
    # Test a few endpoints
    for endpoint in "/v1/account" "/v1/positions"; do
        STATUS=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "http://localhost:$BULLSHIFT_PORT$endpoint" 2>/dev/null)
        if [ "$STATUS" -ge 200 ] && [ "$STATUS" -lt 500 ]; then
            pass "Endpoint $endpoint returned $STATUS"
        else
            fail "Endpoint $endpoint returned $STATUS"
        fi
    done
else
    warn "API server not running on port $BULLSHIFT_PORT — start it first for full verification"
fi

# Summary
echo ""
echo "============================================"
echo -e "Results: ${GREEN}$PASS passed${NC}, ${RED}$FAIL failed${NC}, ${YELLOW}$WARN warnings${NC}"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    echo -e "${RED}Some checks failed. Review output above.${NC}"
    exit 1
elif [ "$WARN" -gt 0 ]; then
    echo -e "${YELLOW}All critical checks passed with warnings.${NC}"
    exit 0
else
    echo -e "${GREEN}All checks passed. BullShift is sandbox-ready.${NC}"
    exit 0
fi
