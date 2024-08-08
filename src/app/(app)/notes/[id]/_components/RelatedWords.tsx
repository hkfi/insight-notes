"use client";

import { RelatedWord } from "@/components/RelatedWord";
import { LoadingSpinner } from "@/components/ui/loading-spinner";
import { useNote } from "@/hooks/queries/notes/use-note";

import { useRelatedWords } from "@/hooks/use-related-words";

export function RelatedWords({ noteId }: { noteId: number }) {
  const { data: relatedWords, isLoading } = useRelatedWords({ noteId });

  const { data: note } = useNote({ id: noteId });

  const wordForTagsInNote = note?.tags.map((tag) => tag.id);

  return (
    <div className="flex flex-col flex-grow">
      <div className="p-2 text-center text-sm text-primary">Related Topics</div>
      <div className="flex gap-2 flex-wrap p-2">
        {isLoading ? (
          <LoadingSpinner />
        ) : (
          relatedWords?.map((word, i) => {
            if (wordForTagsInNote?.includes(word)) return;
            return <RelatedWord key={i} word={word} noteId={noteId} />;
          })
        )}
      </div>
    </div>
  );
}
