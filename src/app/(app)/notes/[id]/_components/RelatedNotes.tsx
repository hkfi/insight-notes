"use client";

import { NoteCard } from "@/components/NoteCard";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useRelatedNotes } from "@/hooks/queries/notes/use-related-notes";

export function RelatedNotes({ noteId }: { noteId: number }) {
  const { relatedNotes } = useRelatedNotes({ noteId });

  return (
    <div className="flex flex-col flex-grow overflow-auto">
      <div className="p-2 text-center text-sm text-primary">Related</div>

      <ScrollArea className="flex-grow">
        <div className="flex flex-col gap-1 p-1">
          {relatedNotes.map((note, i) => {
            if (note?.id === noteId) return null;
            return <NoteCard key={i} note={note} />;
          })}
        </div>
      </ScrollArea>
    </div>
  );
}
