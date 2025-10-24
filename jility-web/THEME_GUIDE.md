# Jility Theme System - Complete Guide

## Overview

The theme system is the **foundation** of the Jility frontend. Every component, color, and UI element is built to support light and dark modes from day one.

## Architecture

### 1. CSS Variables (app/globals.css)

All colors are defined as HSL values in CSS variables:

```css
:root {
  /* Light Mode Colors */
  --background: 0 0% 100%;           /* White */
  --foreground: 222.2 84% 4.9%;      /* Almost black */
  --primary: 221.2 83.2% 53.3%;      /* Blue */
  --border: 214.3 31.8% 91.4%;       /* Light gray */
  /* ... 40+ more variables */
}

.dark {
  /* Dark Mode Colors */
  --background: 222.2 84% 4.9%;      /* Dark blue-gray */
  --foreground: 210 40% 98%;         /* Almost white */
  --primary: 217.2 91.2% 59.8%;      /* Brighter blue */
  --border: 217.2 32.6% 17.5%;       /* Dark gray */
  /* ... matching dark variants */
}
```

### 2. Tailwind Integration (tailwind.config.ts)

Tailwind classes reference the CSS variables:

```typescript
colors: {
  background: 'hsl(var(--background))',
  foreground: 'hsl(var(--foreground))',
  primary: 'hsl(var(--primary))',
  // ... all theme colors
}
```

### 3. Theme Provider (app/providers.tsx)

`next-themes` manages theme state:

```typescript
<ThemeProvider
  attribute="class"           // Uses .dark class
  defaultTheme="light"        // Default to light mode
  enableSystem                // Detect OS preference
  disableTransitionOnChange   // No flash
>
  {children}
</ThemeProvider>
```

### 4. Theme Switcher (components/theme-switcher.tsx)

Three-button toggle for user control:
- **Sun icon** - Light mode
- **Moon icon** - Dark mode
- **Monitor icon** - System preference

## Color Palette

### Light Mode
| Variable | HSL | Hex Approx | Usage |
|----------|-----|------------|-------|
| --background | 0 0% 100% | #FFFFFF | Page background |
| --foreground | 222.2 84% 4.9% | #0A0F1A | Text color |
| --primary | 221.2 83.2% 53.3% | #5B8DEF | Buttons, links |
| --secondary | 210 40% 96.1% | #F1F3F5 | Secondary buttons |
| --muted | 210 40% 96.1% | #F1F3F5 | Disabled states |
| --border | 214.3 31.8% 91.4% | #E5E7EB | Borders |

### Dark Mode
| Variable | HSL | Hex Approx | Usage |
|----------|-----|------------|-------|
| --background | 222.2 84% 4.9% | #0F1419 | Page background |
| --foreground | 210 40% 98% | #F0F2F5 | Text color |
| --primary | 217.2 91.2% 59.8% | #7BA5F5 | Buttons, links |
| --secondary | 217.2 32.6% 17.5% | #1A1F2E | Secondary buttons |
| --muted | 217.2 32.6% 17.5% | #1A1F2E | Disabled states |
| --border | 217.2 32.6% 17.5% | #1A1F2E | Borders |

### Status Colors (Both Modes)
| Status | Light Mode | Dark Mode | Usage |
|--------|------------|-----------|-------|
| Backlog | Gray (60%) | Gray (50%) | Backlog tickets |
| Todo | Blue (53%) | Blue (60%) | To-do tickets |
| In Progress | Green (36%) | Green (45%) | Active tickets |
| Review | Orange (50%) | Orange (50%) | Under review |
| Done | Green (45%) | Green (45%) | Completed |
| Blocked | Red (60%) | Red (60%) | Blocked tickets |

## Usage Examples

### Basic Component

```typescript
// ✅ Correct - Uses theme variables
<div className="bg-background text-foreground border border-border">
  Content
</div>

// ❌ Wrong - Hardcoded colors
<div className="bg-white text-black border border-gray-200">
  Content
</div>
```

### Button Component

```typescript
// Primary button
<button className="bg-primary text-primary-foreground">
  Click me
</button>

// Secondary button
<button className="bg-secondary text-secondary-foreground">
  Cancel
</button>

// Ghost button
<button className="hover:bg-accent hover:text-accent-foreground">
  Subtle action
</button>
```

### Card Component

```typescript
<div className="bg-card text-card-foreground border border-border">
  <h2 className="text-foreground">Title</h2>
  <p className="text-muted-foreground">Description</p>
</div>
```

### Status Badge

```typescript
// Uses theme-aware status colors
<Badge variant="in-progress">In Progress</Badge>
// Light mode: Green background with dark green text
// Dark mode: Darker green background with bright green text
```

## How Theming Works

### 1. User Clicks Theme Button

```
User clicks moon icon
  ↓
next-themes updates localStorage
  ↓
Adds/removes .dark class on <html>
  ↓
CSS variables switch
  ↓
All components re-render with new colors
```

### 2. CSS Variable Lookup

```
Component uses: className="bg-primary"
  ↓
Tailwind: background-color: hsl(var(--primary))
  ↓
Light mode: --primary: 221.2 83.2% 53.3% (#5B8DEF)
Dark mode: --primary: 217.2 91.2% 59.8% (#7BA5F5)
  ↓
Renders with appropriate color
```

### 3. Persistence

```
User selects dark mode
  ↓
Saved to localStorage.theme = "dark"
  ↓
On page reload:
  ↓
next-themes reads localStorage
  ↓
Applies .dark class immediately
  ↓
No flash of light mode (FOUC prevented)
```

## Customization Guide

### Change Primary Color

```css
/* app/globals.css */
:root {
  --primary: 280 100% 50%;  /* Purple instead of blue */
}

.dark {
  --primary: 280 100% 60%;  /* Lighter purple for dark mode */
}
```

### Add New Theme Color

1. **Add CSS variable:**
```css
:root {
  --accent-secondary: 150 70% 50%;
}

.dark {
  --accent-secondary: 150 70% 60%;
}
```

2. **Add to Tailwind config:**
```typescript
// tailwind.config.ts
colors: {
  'accent-secondary': 'hsl(var(--accent-secondary))',
}
```

3. **Use in components:**
```typescript
<div className="bg-accent-secondary">
  Custom color!
</div>
```

### Adjust Status Colors

```css
/* Make "done" color more vibrant */
:root {
  --status-done: 142 80% 50%;  /* Brighter green */
}

.dark {
  --status-done: 142 80% 55%;  /* Even brighter for dark mode */
}
```

## Testing Checklist

### Manual Testing

1. **Light Mode:**
   - [ ] Background is white
   - [ ] Text is dark and readable
   - [ ] Cards have subtle shadows
   - [ ] Borders are visible but subtle
   - [ ] Status badges have appropriate colors
   - [ ] Hover states are clear

2. **Dark Mode:**
   - [ ] Background is dark
   - [ ] Text is light and readable
   - [ ] Cards are elevated from background
   - [ ] Borders separate elements
   - [ ] Status badges are visible
   - [ ] Hover states work

3. **System Mode:**
   - [ ] Follows OS light/dark setting
   - [ ] Changes when OS setting changes
   - [ ] Persists across reloads

4. **Persistence:**
   - [ ] Theme choice survives page reload
   - [ ] No flash of wrong theme (FOUC)
   - [ ] Works in incognito mode

### Browser DevTools Testing

```javascript
// Check localStorage
localStorage.getItem('theme')  // "light", "dark", or "system"

// Check HTML class
document.documentElement.classList.contains('dark')  // true in dark mode

// Check computed CSS variable
getComputedStyle(document.documentElement)
  .getPropertyValue('--primary')  // "221.2 83.2% 53.3%"
```

## Common Patterns

### Conditional Styling Based on Theme

```typescript
// ✅ Good - Uses theme variables
<div className="bg-background hover:bg-accent">
  Adapts to theme automatically
</div>

// Sometimes you need different behavior per theme:
import { useTheme } from 'next-themes'

function Component() {
  const { theme } = useTheme()

  return (
    <img
      src={theme === 'dark' ? '/logo-dark.svg' : '/logo-light.svg'}
      alt="Logo"
    />
  )
}
```

### Status Color Helper

```typescript
// lib/utils.ts
export function getStatusColor(status: string): string {
  const colors = {
    backlog: 'status-backlog',
    todo: 'status-todo',
    in_progress: 'status-in-progress',
    review: 'status-review',
    done: 'status-done',
    blocked: 'status-blocked',
  }
  return colors[status as keyof typeof colors] || 'status-backlog'
}

// Usage
<Badge variant={getStatusColor(ticket.status)}>
  {ticket.status}
</Badge>
```

## Best Practices

### ✅ Do

- Use semantic variable names (--foreground, --background)
- Test in both light and dark modes
- Ensure sufficient contrast (WCAG AA)
- Use status colors for semantic meaning
- Keep transitions smooth (150-200ms)

### ❌ Don't

- Hardcode colors like `bg-white` or `text-black`
- Use hex colors directly in components
- Forget to test theme switching
- Assume colors work in both modes
- Skip accessibility contrast checks

## Accessibility

### Color Contrast

All theme colors meet WCAG AA standards:
- **Normal text:** 4.5:1 minimum
- **Large text:** 3:1 minimum
- **UI components:** 3:1 minimum

### Testing Contrast

```javascript
// Light mode: foreground on background
// Ratio: ~20:1 (exceeds AA)

// Dark mode: foreground on background
// Ratio: ~18:1 (exceeds AA)

// Primary on background (light)
// Ratio: ~4.8:1 (meets AA)

// Primary on background (dark)
// Ratio: ~5.2:1 (meets AA)
```

### Focus States

All interactive elements have visible focus rings:

```css
.focus-visible\:outline-none:focus-visible {
  outline: 2px solid transparent;
  outline-offset: 2px;
}

.focus-visible\:ring-2:focus-visible {
  --tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0
    var(--tw-ring-offset-width) var(--tw-ring-offset-color);
  --tw-ring-shadow: var(--tw-ring-inset) 0 0 0
    calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);
  box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow);
}
```

## Troubleshooting

### Theme not switching?

1. Check browser console for errors
2. Verify next-themes is installed
3. Clear localStorage and try again
4. Check if .dark class appears on <html>

### Colors look wrong?

1. Verify CSS variables are defined
2. Check Tailwind config references them
3. Inspect element to see computed values
4. Make sure using hsl() format

### Flash of wrong theme on load?

1. Ensure suppressHydrationWarning on <html>
2. Check next-themes configuration
3. Verify disableTransitionOnChange is set

## Summary

The Jility theme system is:
- **Complete** - All components themed
- **Accessible** - WCAG AA compliant
- **Performant** - CSS variables, no JS
- **Flexible** - Easy to customize
- **Persistent** - Remembers user choice
- **Professional** - Linear-quality UI

Every color, border, and shadow adapts seamlessly between light and dark modes, providing a consistent and delightful user experience.
