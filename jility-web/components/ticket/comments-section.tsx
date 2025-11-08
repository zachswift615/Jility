'use client'

import { useState } from 'react'
import type { Comment } from '@/lib/types'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import { CommentItem } from './comment-item'
import { useToast } from '@/hooks/use-toast'
import { MessageSquare, Send } from 'lucide-react'

interface CommentsSectionProps {
  comments: Comment[]
  currentUser?: string
  onAddComment: (content: string) => Promise<void>
  onEditComment?: (id: string, content: string) => Promise<void>
  onDeleteComment?: (id: string) => Promise<void>
}

export function CommentsSection({
  comments,
  currentUser = 'system',
  onAddComment,
  onEditComment,
  onDeleteComment,
}: CommentsSectionProps) {
  const { toast } = useToast()
  const [newComment, setNewComment] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!newComment.trim()) return

    setIsSubmitting(true)
    try {
      await onAddComment(newComment.trim())
      setNewComment('')
    } catch (error) {
      console.error('Failed to add comment:', error)
      toast({
        title: 'Failed to add comment',
        description: 'Please try again',
        variant: 'destructive',
      })
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="mt-6 md:mt-8 space-y-4 md:space-y-6">
      <div className="flex items-center gap-2">
        <MessageSquare className="h-4 w-4 md:h-5 md:w-5 text-muted-foreground" />
        <h2 className="text-base md:text-lg font-semibold">
          Comments ({comments.length})
        </h2>
      </div>

      {/* Comment List */}
      <div className="space-y-3 md:space-y-4">
        {comments.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            No comments yet. Start the conversation!
          </p>
        ) : (
          comments.map((comment) => (
            <CommentItem
              key={comment.id}
              comment={comment}
              currentUser={currentUser}
              onEdit={onEditComment}
              onDelete={onDeleteComment}
            />
          ))
        )}
      </div>

      {/* New Comment Form */}
      <form onSubmit={handleSubmit} className="space-y-2 md:space-y-3">
        <Textarea
          placeholder="Add a comment..."
          value={newComment}
          onChange={(e) => setNewComment(e.target.value)}
          className="min-h-20 md:min-h-24 text-sm"
          disabled={isSubmitting}
        />
        <div className="flex justify-end">
          <Button type="submit" disabled={isSubmitting || !newComment.trim()} size="sm" className="text-xs md:text-sm">
            {isSubmitting ? (
              'Posting...'
            ) : (
              <>
                <Send className="h-4 w-4 mr-2" />
                Comment
              </>
            )}
          </Button>
        </div>
      </form>
    </div>
  )
}
