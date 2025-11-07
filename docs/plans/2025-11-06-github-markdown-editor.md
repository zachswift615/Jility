# GitHub-Style Markdown Editor with Write/Preview Tabs

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance ticket description editor with GitHub-style tabs to switch between Write and Preview modes

**Architecture:** Existing TicketDescription component already has react-markdown with GFM support. Add tabs component with "Write" and "Preview" tabs. Write tab shows textarea, Preview tab shows rendered markdown. Both visible during editing.

**Tech Stack:**
- Frontend: Next.js 14, React, TypeScript
- Markdown: react-markdown, remark-gfm, rehype-highlight (already installed)
- UI: shadcn/ui Tabs component

---

## Task 1: Install shadcn Tabs Component

**Files:**
- Will create: `jility-web/components/ui/tabs.tsx`

**Step 1: Install tabs component**

```bash
cd jility-web
npx shadcn@latest add tabs
```

Expected: Creates `components/ui/tabs.tsx`

**Step 2: Verify installation**

```bash
ls -la components/ui/tabs.tsx
```

Expected: File exists

**Step 3: Build to verify**

```bash
npm run build
```

Expected: Build succeeds

**Step 4: Commit**

```bash
git add components/ui/tabs.tsx
git commit -m "feat: add shadcn tabs component"
```

---

## Task 2: Create Markdown Preview Component

**Files:**
- Create: `jility-web/components/ui/markdown-preview.tsx`

**Step 1: Create reusable markdown preview component**

```typescript
'use client'

import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import rehypeHighlight from 'rehype-highlight'
import 'highlight.js/styles/github-dark.css'

interface MarkdownPreviewProps {
  content: string
  className?: string
}

export function MarkdownPreview({ content, className = '' }: MarkdownPreviewProps) {
  return (
    <div className={`prose prose-sm md:prose max-w-none dark:prose-invert ${className}`}>
      <ReactMarkdown remarkPlugins={[remarkGfm]} rehypePlugins={[rehypeHighlight]}>
        {content || '*No content*'}
      </ReactMarkdown>
    </div>
  )
}
```

**Step 2: Build**

```bash
npm run build
```

**Step 3: Commit**

```bash
git add jility-web/components/ui/markdown-preview.tsx
git commit -m "feat: add reusable markdown preview component"
```

---

## Task 3: Enhance TicketDescription with Tabs

**Files:**
- Modify: `jility-web/components/ticket/ticket-description.tsx`

**Step 1: Update imports**

Replace the current imports with:

```typescript
'use client'

import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { MarkdownPreview } from '@/components/ui/markdown-preview'
import { Edit, Check, X } from 'lucide-react'
```

**Step 2: Add tab state**

In the component, after the existing state declarations:

```typescript
const [activeTab, setActiveTab] = useState<'write' | 'preview'>('write')
```

**Step 3: Replace the editing section**

Replace lines 49-67 (the editing `<div className="space-y-2">` section) with:

```typescript
{isEditing ? (
  <div className="space-y-2">
    <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as 'write' | 'preview')}>
      <TabsList className="grid w-full max-w-[400px] grid-cols-2">
        <TabsTrigger value="write">Write</TabsTrigger>
        <TabsTrigger value="preview">Preview</TabsTrigger>
      </TabsList>

      <TabsContent value="write" className="mt-4">
        <Textarea
          value={value}
          onChange={(e) => setValue(e.target.value)}
          className="min-h-[300px] font-mono text-sm"
          placeholder="Write your description in Markdown...

**Markdown Tips:**
- **Bold** with **text**
- *Italic* with *text*
- `Code` with backticks
- [Links](url)
- # Headings with #
- - Lists with -
- ```code blocks```"
          autoFocus
        />
      </TabsContent>

      <TabsContent value="preview" className="mt-4">
        <div className="min-h-[300px] p-4 bg-muted rounded-lg border border-border">
          <MarkdownPreview content={value} />
        </div>
      </TabsContent>
    </Tabs>

    <div className="flex gap-2 pt-2">
      <Button size="sm" onClick={handleSave}>
        <Check className="h-4 w-4 mr-2" />
        Save
      </Button>
      <Button size="sm" variant="outline" onClick={handleCancel}>
        <X className="h-4 w-4 mr-2" />
        Cancel
      </Button>
    </div>
  </div>
) : (
  <div className="p-3 md:p-4 bg-muted rounded-lg border border-border">
    <MarkdownPreview content={description} />
  </div>
)}
```

**Step 4: Update handleCancel to reset tab**

Modify the `handleCancel` function:

```typescript
const handleCancel = () => {
  setValue(description)
  setIsEditing(false)
  setActiveTab('write')
}
```

**Step 5: Update handleSave to reset tab**

Modify the `handleSave` function:

```typescript
const handleSave = () => {
  if (value.trim() && value !== description && onUpdate) {
    onUpdate(value.trim())
  }
  setIsEditing(false)
  setActiveTab('write')
}
```

**Step 6: Build and verify**

```bash
npm run build
```

Expected: Build succeeds

**Step 7: Commit**

```bash
git add jility-web/components/ticket/ticket-description.tsx
git commit -m "feat: add GitHub-style write/preview tabs to ticket description"
```

---

## Task 4: Apply Same Pattern to Ticket Creation

**Files:**
- Modify: `jility-web/components/kanban/create-ticket-dialog.tsx` (if exists)
- Or modify: `jility-web/components/board/create-ticket-form.tsx` (find the right file)

**Step 1: Find ticket creation component**

```bash
find jility-web/components -name "*create*ticket*" -o -name "*ticket*form*"
```

**Step 2: Apply the same tabs pattern**

If the component has a description textarea, replace it with the tabbed editor:

```typescript
<Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as 'write' | 'preview')}>
  <TabsList className="grid w-full max-w-[400px] grid-cols-2">
    <TabsTrigger value="write">Write</TabsTrigger>
    <TabsTrigger value="preview">Preview</TabsTrigger>
  </TabsList>

  <TabsContent value="write" className="mt-4">
    <Textarea
      value={description}
      onChange={(e) => setDescription(e.target.value)}
      className="min-h-[200px] font-mono text-sm"
      placeholder="Describe the ticket (Markdown supported)..."
    />
  </TabsContent>

  <TabsContent value="preview" className="mt-4">
    <div className="min-h-[200px] p-4 bg-muted rounded-lg border border-border">
      <MarkdownPreview content={description} />
    </div>
  </TabsContent>
</Tabs>
```

**Step 3: Build and commit**

```bash
npm run build
git add jility-web/components/[...updated file...]
git commit -m "feat: add markdown tabs to ticket creation form"
```

---

## Task 5: Add Markdown Help Tooltip (Optional Enhancement)

**Files:**
- Create: `jility-web/components/ui/markdown-help-popover.tsx`

**Step 1: Install popover if not already installed**

```bash
npx shadcn@latest add popover
```

**Step 2: Create markdown help component**

```typescript
'use client'

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Button } from '@/components/ui/button'
import { HelpCircle } from 'lucide-react'

export function MarkdownHelpPopover() {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="ghost" size="sm">
          <HelpCircle className="h-4 w-4 mr-2" />
          Markdown Help
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="space-y-3">
          <h4 className="font-semibold">Markdown Formatting</h4>
          <div className="text-sm space-y-2">
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                **bold**
              </code>
              <span className="ml-2">for <strong>bold text</strong></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                *italic*
              </code>
              <span className="ml-2">for <em>italic text</em></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                `code`
              </code>
              <span className="ml-2">for <code>inline code</code></span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                # Heading
              </code>
              <span className="ml-2">for headings</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                [link](url)
              </code>
              <span className="ml-2">for links</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                - item
              </code>
              <span className="ml-2">for bullet lists</span>
            </div>
            <div>
              <code className="text-xs bg-muted px-1 py-0.5 rounded">
                ```code```
              </code>
              <span className="ml-2">for code blocks</span>
            </div>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  )
}
```

**Step 3: Add help button to tabs**

In `ticket-description.tsx`, add the help button next to the TabsList:

```typescript
<div className="flex items-center justify-between">
  <TabsList className="grid w-full max-w-[400px] grid-cols-2">
    <TabsTrigger value="write">Write</TabsTrigger>
    <TabsTrigger value="preview">Preview</TabsTrigger>
  </TabsList>
  <MarkdownHelpPopover />
</div>
```

**Step 4: Build and commit**

```bash
npm run build
git add jility-web/components/ui/markdown-help-popover.tsx jility-web/components/ticket/ticket-description.tsx
git commit -m "feat: add markdown help popover"
```

---

## Task 6: Improve Markdown Styling

**Files:**
- Modify: `jility-web/app/globals.css`

**Step 1: Add custom prose styles**

Add at the end of `globals.css`:

```css
/* Enhanced markdown prose styles */
.prose code {
  @apply bg-muted px-1 py-0.5 rounded text-sm;
}

.prose pre {
  @apply bg-muted border border-border;
}

.prose blockquote {
  @apply border-l-4 border-primary pl-4 italic;
}

.prose table {
  @apply border-collapse border border-border;
}

.prose th,
.prose td {
  @apply border border-border px-4 py-2;
}

.prose th {
  @apply bg-muted font-semibold;
}

.prose a {
  @apply text-primary underline-offset-4 hover:underline;
}

.prose h1,
.prose h2,
.prose h3,
.prose h4,
.prose h5,
.prose h6 {
  @apply font-semibold text-foreground;
}

.prose ul,
.prose ol {
  @apply my-4;
}

.prose li {
  @apply my-1;
}
```

**Step 2: Build**

```bash
npm run build
```

**Step 3: Commit**

```bash
git add jility-web/app/globals.css
git commit -m "style: enhance markdown prose styling"
```

---

## Task 7: Test in Docker

**Step 1: Rebuild and start containers**

```bash
docker-compose down
docker-compose up --build -d
```

**Step 2: Manual testing checklist**

- [ ] Create a new ticket with markdown in description
- [ ] Click "Edit" on ticket description
- [ ] Switch between Write and Preview tabs
- [ ] Verify markdown renders correctly in Preview tab
- [ ] Save changes and verify rendered output
- [ ] Test with various markdown syntax:
  - [ ] Headings (# ## ###)
  - [ ] Bold (**text**)
  - [ ] Italic (*text*)
  - [ ] Code (`inline` and ```blocks```)
  - [ ] Links ([text](url))
  - [ ] Lists (- bullets, 1. numbered)
  - [ ] Tables (| col | col |)
  - [ ] Blockquotes (> text)

**Step 3: Test on mobile**

- [ ] Edit description on mobile device
- [ ] Verify tabs are touch-friendly
- [ ] Verify textarea is usable

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat: GitHub-style markdown editor complete"
git push
```

---

## Future Enhancements (Not in this plan)

- Drag-and-drop image upload
- Markdown shortcuts (Ctrl+B for bold, etc.)
- Split-screen mode (write and preview side-by-side)
- Markdown templates/snippets
- Emoji picker
- @mention autocomplete
- Task list checkboxes (- [ ] item)
