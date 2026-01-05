#!/bin/bash
# Debug WebSocket responses
echo '{"corrId":"1","cmd":"/contacts"}' | websocat ws://127.0.0.1:5225 | head -1 | jq .
