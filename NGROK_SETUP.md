# Ngrok Setup for External Testing

## Problem
When testing Jility through ngrok from a cellular network, the login fails. This happens because the frontend (port 3901) and backend (port 3900) are separate services, and external clients can only reach one of them through ngrok.

## Solution: Next.js API Proxy

The frontend uses Next.js rewrites to proxy `/api/*` requests to the backend. This allows:
- ✅ Single ngrok tunnel (to frontend on port 3901)
- ✅ External clients can access both UI and API through one URL
- ✅ Backend stays on localhost:3900

## Setup Steps

### Step 1: Start Servers with Ngrok Config

Start the dev servers using the ngrok configuration:

```bash
./dev.sh start --config=.env.ngrok
```

This will:
- Load environment variables from `.env.ngrok` (without modifying `.env.local`)
- Start both backend (3900) and frontend (3901)
- Keep both config files intact

### Step 2: Start Ngrok (Frontend Port)

Start ngrok pointing to your **frontend** (port 3901):

```bash
ngrok http 3901
```

Copy the `https://` URL that ngrok provides (e.g., `https://guilelessly-nonrhythmic-jo.ngrok-free.dev`)

### Step 3: Switch Back to Local Dev

When you're done testing with ngrok, simply restart without the config flag:

```bash
./dev.sh restart
```

This will restore the default `.env.local` configuration for local development.

### Step 4: Access from External Network

Now you can access the ngrok URL from any network (WiFi or cellular):

```
https://your-ngrok-url.ngrok-free.dev
```

**How it works:**
1. Browser loads frontend from ngrok URL
2. Frontend makes fetch to `/api/auth/login` (relative URL)
3. Next.js server receives request
4. Next.js rewrites `/api/*` to `http://localhost:3900/api/*`
5. Backend processes request
6. Response flows back through Next.js to browser

## Configuration Files

The project includes two environment configurations:

**`.env.local` (default)** - For local development:
```bash
NEXT_PUBLIC_API_URL=/api
BACKEND_URL=http://localhost:3900
```

**`.env.ngrok`** - For ngrok/external testing (same as local):
```bash
NEXT_PUBLIC_API_URL=/api
BACKEND_URL=http://localhost:3900
```

Both configs use the same settings because Next.js proxies API requests server-side. You can customize `.env.ngrok` if you need different settings for external testing.

## Quick Reference

```bash
# Start with default local config
./dev.sh start

# Start with ngrok config
./dev.sh start --config=.env.ngrok

# Restart with same config
./dev.sh restart --config=.env.ngrok

# Switch back to local
./dev.sh restart

# Check status
./dev.sh status

# Stop servers
./dev.sh stop
```

## Troubleshooting

### Still seeing "Load failed"?

1. **Check the browser console** for the actual error
2. **Verify the API URL** by checking the Network tab in DevTools
3. **Check backend logs** to see if requests are reaching the server
4. **Try the API directly**: Visit `https://your-ngrok-url.ngrok-free.dev/api/projects` in your browser

### CORS errors?

The backend is configured to allow all origins (`allow_origin(Any)`), but if you see CORS errors:

1. Check that ngrok isn't adding extra headers
2. Verify the backend is running on the port ngrok is tunneling to
3. Check backend logs for rejected requests

### "Invalid host header" from Next.js?

If you're accessing the Next.js dev server through ngrok and see this error, you need to allow ngrok hosts:

```bash
# In jility-web/.env.local
NEXT_PUBLIC_API_URL=https://your-ngrok-url.ngrok-free.dev/api
```

Then rebuild/restart.

## Architecture Notes

```
[Mobile on Cellular]
    ↓
[Ngrok Cloud]
    ↓
[Your Computer - Backend:3900]
    ↓ (serves API at /api/*)
[Response]
```

The frontend (Next.js) is either:
- Served from the same ngrok URL (if you're serving the built frontend through the backend)
- OR making API calls to the ngrok URL (if you're running Next.js separately)

**Current setup**: We're running Next.js separately, so it makes fetch requests to the ngrok URL.
