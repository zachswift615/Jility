# Jility Web - Quick Start Guide

## Prerequisites

1. **Jility Server Running**
   - The backend must be running at `http://localhost:3000`
   - Start it with: `cd /home/user/Jility/jility-server && cargo run`

2. **Node.js 18+**
   - Check version: `node --version`

## Installation (5 minutes)

### Step 1: Install Dependencies

```bash
cd /home/user/Jility/jility-web
npm install
```

### Step 2: Create Environment File

```bash
cp .env.example .env
```

The default values work with local development:
```
NEXT_PUBLIC_API_URL=http://localhost:3000/api
NEXT_PUBLIC_WS_URL=ws://localhost:3000/ws
```

### Step 3: Start Development Server

```bash
npm run dev
```

The app will be available at **http://localhost:3001**

## First-Time Setup

### 1. Create Sample Data (Optional)

If your database is empty, create some sample tickets using the API:

```bash
# Create a ticket
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Build authentication system",
    "description": "Implement JWT-based authentication with refresh tokens",
    "status": "todo",
    "story_points": 8,
    "assignees": ["alice", "agent-1"],
    "labels": ["backend", "security"]
  }'

# Create another ticket
curl -X POST http://localhost:3000/api/tickets \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Design landing page",
    "description": "Create a beautiful hero section with call-to-action",
    "status": "in_progress",
    "story_points": 5,
    "assignees": ["bob"],
    "labels": ["frontend", "design"]
  }'
```

### 2. Test Theme Switching

1. Open the app at http://localhost:3001
2. Look for the theme switcher in the top-right corner of the navbar
3. Click the **sun icon** for light mode
4. Click the **moon icon** for dark mode
5. Click the **monitor icon** for system preference
6. Refresh the page - your choice should persist!

### 3. Test Kanban Board

1. Navigate to the Board page (default view)
2. You should see six columns: Backlog, Todo, In Progress, Review, Done, Blocked
3. **Drag a ticket** from one column to another
4. The status should update immediately
5. Open another browser window and drag a ticket - both windows update in real-time!

### 4. Test Ticket Detail View

1. Click on any ticket card
2. Try these features:
   - **Click the title** to edit it
   - **Click "Edit"** on the description to modify it (supports Markdown!)
   - **Add a comment** at the bottom
   - **View the activity timeline** on the right sidebar

### 5. Test Command Palette

1. Press **⌘K** (Mac) or **Ctrl+K** (Windows/Linux)
2. Type to search for tickets
3. Use arrow keys to navigate results
4. Press Enter to open a ticket
5. Press Esc to close the palette

### 6. Test Agent Dashboard

1. Navigate to **Agents** in the navbar
2. See metrics for all AI agents
3. View per-agent statistics
4. Click on any ticket to view details

## Testing Theme System

### Manual Theme Testing

1. **Light Mode:**
   - Background should be white (#FFFFFF)
   - Text should be dark (#0A0F1A)
   - Cards should have subtle shadows
   - Status badges should have light backgrounds

2. **Dark Mode:**
   - Background should be dark (#0F1419)
   - Text should be light (#F0F2F5)
   - Cards should have darker backgrounds
   - Status badges should have adjusted colors

3. **System Preference:**
   - Set your OS to dark mode
   - Select "System" in theme switcher
   - App should switch to dark mode
   - Change OS to light mode
   - App should switch to light mode

### Browser DevTools Testing

1. Open DevTools (F12)
2. Go to Elements tab
3. Find `<html>` element
4. In dark mode, it should have `class="dark"`
5. In light mode, it should NOT have the `class="dark"`
6. Check CSS variables:
   ```
   :root {
     --background: 0 0% 100%;  /* Light mode */
   }

   .dark {
     --background: 222.2 84% 4.9%;  /* Dark mode */
   }
   ```

### Persistence Testing

1. Select dark mode
2. Refresh the page
3. Dark mode should still be active
4. Check localStorage:
   ```javascript
   localStorage.getItem('theme')  // Should be "dark"
   ```

## Common Issues & Solutions

### Issue: "Failed to fetch tickets"

**Solution:** Make sure the Jility server is running:
```bash
cd /home/user/Jility/jility-server
cargo run
```

### Issue: "WebSocket connection failed"

**Solution:** Check that the server is running and WebSocket endpoint is available:
```bash
# Should return 200 OK
curl http://localhost:3000/api/tickets
```

### Issue: Theme not switching

**Solution:**
1. Clear browser cache
2. Check for JavaScript errors in DevTools Console
3. Verify `next-themes` is installed: `npm list next-themes`

### Issue: Drag-and-drop not working

**Solution:**
1. Make sure you're dragging from the grip icon (six dots)
2. Check that `@dnd-kit` packages are installed
3. Try refreshing the page

### Issue: Markdown not rendering

**Solution:**
1. Verify description contains valid Markdown
2. Check that `react-markdown` is installed
3. Look for errors in DevTools Console

## Development Tips

### Hot Reload

Next.js has hot module replacement (HMR) built-in. Changes to files will automatically reload in the browser.

### Inspecting WebSocket Messages

Open DevTools Console and look for:
```
WebSocket connected
WebSocket message: { type: 'ticket_created', ... }
```

### Tailwind IntelliSense

If using VS Code, install the **Tailwind CSS IntelliSense** extension for autocomplete of class names.

### Type Checking

Run TypeScript type checking:
```bash
npm run build
```

This will show any type errors before deployment.

## Production Build

### Build the App

```bash
npm run build
```

Expected output:
```
✓ Compiled successfully
✓ Linting and checking validity of types
✓ Generating static pages (6/6)
✓ Finalizing page optimization

Route (app)                              Size     First Load JS
┌ ○ /                                    137 B          87.1 kB
├ ○ /agents                              3.58 kB         106 kB
├ ○ /board                               19.8 kB         122 kB
└ ƒ /ticket/[id]                         96.9 kB         192 kB
```

### Start Production Server

```bash
npm start
```

The production build will be served at http://localhost:3001

## Next Steps

1. **Customize Theme:** Edit `app/globals.css` to change colors
2. **Add Features:** Extend components in `components/`
3. **Write Tests:** Add Jest tests for components
4. **Deploy:** Deploy to Vercel, Netlify, or your preferred platform

## Useful Commands

```bash
# Development
npm run dev          # Start dev server
npm run build        # Build for production
npm start            # Start production server

# Linting
npm run lint         # Run ESLint

# Package Management
npm install          # Install dependencies
npm update           # Update dependencies
```

## Resources

- **Next.js Docs:** https://nextjs.org/docs
- **Tailwind CSS:** https://tailwindcss.com/docs
- **next-themes:** https://github.com/pacocoursey/next-themes
- **@dnd-kit:** https://docs.dndkit.com
- **react-markdown:** https://github.com/remarkjs/react-markdown

## Support

If you encounter issues:
1. Check the browser DevTools Console for errors
2. Verify the Jility server is running
3. Review `IMPLEMENTATION_SUMMARY.md` for architecture details
4. Check `README.md` for API integration docs

## Success Checklist

After completing this guide, you should be able to:

- ✅ View the Kanban board with tickets
- ✅ Switch between light and dark themes
- ✅ Drag tickets between columns
- ✅ View ticket details
- ✅ Edit ticket title and description
- ✅ Add comments
- ✅ Use the command palette (⌘K)
- ✅ View agent dashboard
- ✅ See real-time updates

**Enjoy using Jility!** 🎉
