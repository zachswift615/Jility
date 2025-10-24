# Jility Web - Next.js Frontend

Beautiful, Linear-inspired project management UI for Jility with built-in theme support.

## Features

- **Theme System** - Light, dark, and system preference modes
- **Kanban Board** - Drag-and-drop ticket management with real-time updates
- **Ticket Detail View** - Rich markdown rendering, comments, and activity timeline
- **Command Palette** - ⌘K for keyboard-driven workflows
- **Real-time Updates** - WebSocket integration for live collaboration
- **Agent Dashboard** - Track AI agent activity and productivity
- **Responsive Design** - Works beautifully on all screen sizes

## Tech Stack

- **Next.js 14** - App Router with React Server Components
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first styling with CSS variables for theming
- **next-themes** - Theme management (light/dark/system)
- **@dnd-kit** - Accessible drag-and-drop for Kanban board
- **cmdk** - Command palette component
- **react-markdown** - Markdown rendering with syntax highlighting
- **lucide-react** - Beautiful icon library

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Jility server running at `http://localhost:3000`

### Installation

1. Install dependencies:

```bash
npm install
```

2. Create environment file:

```bash
cp .env.example .env
```

3. Start the development server:

```bash
npm run dev
```

4. Open [http://localhost:3001](http://localhost:3001) in your browser

## Theme System

The theme system is built with CSS variables and `next-themes`, providing seamless light/dark mode switching.

### Theme Configuration

All theme colors are defined in `app/globals.css` using HSL values:

```css
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  /* ... */
}

.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  /* ... */
}
```

### Using Theme Colors

All components use Tailwind classes that reference theme CSS variables:

```tsx
<div className="bg-background text-foreground">
  <button className="bg-primary text-primary-foreground">
    Click me
  </button>
</div>
```

### Theme Switcher

The theme switcher component (`components/theme-switcher.tsx`) allows users to switch between:
- **Light mode** - Clean, bright interface
- **Dark mode** - Eye-friendly dark interface
- **System** - Follows OS preference

### Customizing Colors

To customize the theme, edit the CSS variables in `app/globals.css`. All components will automatically update.

## Project Structure

```
jility-web/
├── app/
│   ├── layout.tsx              # Root layout with theme provider
│   ├── page.tsx                # Home (redirects to /board)
│   ├── globals.css             # Theme CSS variables
│   ├── providers.tsx           # Theme provider wrapper
│   ├── board/                  # Kanban board view
│   ├── ticket/[id]/            # Ticket detail view
│   └── agents/                 # Agent dashboard
│
├── components/
│   ├── ui/                     # Design system components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── badge.tsx
│   │   ├── input.tsx
│   │   └── ...
│   ├── kanban/                 # Kanban board components
│   │   ├── board.tsx
│   │   ├── column.tsx
│   │   └── ticket-card.tsx
│   ├── ticket/                 # Ticket detail components
│   │   ├── ticket-header.tsx
│   │   ├── ticket-description.tsx
│   │   ├── comments-section.tsx
│   │   └── activity-timeline.tsx
│   ├── command-palette.tsx     # ⌘K command palette
│   ├── theme-switcher.tsx      # Theme toggle
│   └── layout/
│       └── navbar.tsx
│
├── lib/
│   ├── api.ts                  # API client
│   ├── websocket.ts            # WebSocket client
│   ├── types.ts                # TypeScript types
│   └── utils.ts                # Utilities
│
└── public/
```

## Key Features

### Kanban Board

The Kanban board (`app/board/page.tsx`) provides:
- Drag-and-drop ticket movement between columns
- Real-time updates via WebSocket
- Keyboard navigation alternative
- Responsive design with horizontal scroll on mobile
- Status columns: Backlog, Todo, In Progress, Review, Done, Blocked

### Ticket Detail View

The ticket detail view (`app/ticket/[id]/page.tsx`) includes:
- Inline title editing
- Markdown description with syntax highlighting
- Comments section with markdown support
- Activity timeline showing all changes
- Linked commits and dependencies
- Assignees and labels management

### Command Palette

Press **⌘K** (Cmd+K) or **Ctrl+K** to open the command palette:
- Search tickets by title
- Quick actions (create ticket, etc.)
- Keyboard navigation
- Fuzzy search

### Real-time Updates

WebSocket integration provides live updates for:
- Ticket creation
- Status changes
- Comments
- Description edits

## API Integration

The API client (`lib/api.ts`) provides type-safe methods for all Jility API endpoints:

```typescript
import { api } from '@/lib/api'

// List tickets
const tickets = await api.listTickets({ status: 'todo' })

// Create ticket
const ticket = await api.createTicket({
  title: 'New Feature',
  description: 'Build something cool',
  status: 'todo',
})

// Update status
await api.updateTicketStatus(ticketId, 'in_progress')

// Add comment
await api.createComment(ticketId, 'Looking good!')
```

## Building for Production

```bash
npm run build
npm start
```

## Environment Variables

- `NEXT_PUBLIC_API_URL` - Jility API base URL (default: `http://localhost:3000/api`)
- `NEXT_PUBLIC_WS_URL` - WebSocket URL (default: `ws://localhost:3000/ws`)

## Design System

All UI components are built with:
- Consistent spacing and typography
- Theme-aware colors using CSS variables
- Accessible focus states
- Smooth transitions
- Responsive layouts

### Component Library

The `components/ui/` directory contains reusable design system components:
- **Button** - Primary, secondary, ghost, outline variants
- **Card** - Content containers with header/footer
- **Badge** - Status indicators (uses theme colors)
- **Input/Textarea** - Form fields
- **Avatar** - User/agent avatars

All components use the `cn()` utility for class merging:

```typescript
import { cn } from '@/lib/utils'

<div className={cn('base-class', variant && 'variant-class', className)} />
```

## Keyboard Shortcuts

- **⌘K** / **Ctrl+K** - Open command palette
- **Esc** - Close dialogs/modals
- **Enter** - Confirm inline edits

## License

MIT
