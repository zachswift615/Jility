#!/bin/bash

# Start ngrok tunnel and configure Next.js to use it
# Usage: ./start-ngrok.sh

echo "Starting ngrok tunnel on port 3901..."
ngrok http 3901 > /dev/null 2>&1 &
NGROK_PID=$!

# Wait for ngrok to start
sleep 3

# Get the ngrok URL
NGROK_URL=$(curl -s http://localhost:4040/api/tunnels | grep -o '"public_url":"https://[^"]*' | grep -o 'https://[^"]*' | head -1)

if [ -z "$NGROK_URL" ]; then
    echo "❌ Failed to get ngrok URL. Is ngrok running?"
    exit 1
fi

echo "✅ Ngrok tunnel active: $NGROK_URL"
echo ""
echo "To test from external network:"
echo "1. Update jility-web/.env.local with:"
echo "   NEXT_PUBLIC_API_URL=${NGROK_URL}/api"
echo ""
echo "2. Rebuild the Next.js app:"
echo "   cd jility-web && npm run build && npm start"
echo ""
echo "3. Access the ngrok URL in your browser"
echo ""
echo "Press Ctrl+C to stop ngrok"

# Keep script running
wait $NGROK_PID
