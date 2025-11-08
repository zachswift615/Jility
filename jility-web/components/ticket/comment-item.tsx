'use client'

import { useState } from 'react'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { formatDate } from '@/lib/utils'
import { MoreHorizontal, Pencil, Trash2 } from 'lucide-react'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface CommentItemProps {
  comment: {
    id: string
    author: string
    content: string
    created_at: string
    updated_at?: string
  }
  currentUser: string
  onEdit?: (id: string, content: string) => Promise<void>
  onDelete?: (id: string) => Promise<void>
}

export function CommentItem({ comment, currentUser, onEdit, onDelete }: CommentItemProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [editedContent, setEditedContent] = useState(comment.content)
  const [isSaving, setIsSaving] = useState(false)

  const isAuthor = comment.author === currentUser
  const isEdited = comment.updated_at && comment.updated_at !== comment.created_at

  const handleSave = async () => {
    if (!editedContent.trim() || !onEdit) return

    setIsSaving(true)
    try {
      await onEdit(comment.id, editedContent)
      setIsEditing(false)
    } catch (error) {
      console.error('Failed to edit comment:', error)
    } finally {
      setIsSaving(false)
    }
  }

  const handleDelete = async () => {
    if (!onDelete) return
    if (!confirm('Delete this comment?')) return

    try {
      await onDelete(comment.id)
    } catch (error) {
      console.error('Failed to delete comment:', error)
    }
  }

  return (
    <div className="flex gap-3 group">
      <Avatar className="h-8 w-8">
        <AvatarFallback className="text-xs">
          {comment.author.slice(0, 2).toUpperCase()}
        </AvatarFallback>
      </Avatar>

      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2 mb-1">
          <span className="font-medium text-sm">{comment.author}</span>
          <span className="text-xs text-muted-foreground">
            {formatDate(comment.created_at)}
            {isEdited && ' (edited)'}
          </span>
        </div>

        {isEditing ? (
          <div className="space-y-2">
            <Textarea
              value={editedContent}
              onChange={(e) => setEditedContent(e.target.value)}
              className="min-h-20"
            />
            <div className="flex gap-2">
              <Button size="sm" onClick={handleSave} disabled={isSaving}>
                Save
              </Button>
              <Button
                size="sm"
                variant="outline"
                onClick={() => {
                  setEditedContent(comment.content)
                  setIsEditing(false)
                }}
                disabled={isSaving}
              >
                Cancel
              </Button>
            </div>
          </div>
        ) : (
          <div className="prose prose-sm max-w-none">
            <p className="text-sm whitespace-pre-wrap">{comment.content}</p>
          </div>
        )}
      </div>

      {isAuthor && !isEditing && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0 opacity-0 group-hover:opacity-100"
            >
              <MoreHorizontal className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => setIsEditing(true)}>
              <Pencil className="h-4 w-4 mr-2" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem onClick={handleDelete} className="text-destructive">
              <Trash2 className="h-4 w-4 mr-2" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  )
}
