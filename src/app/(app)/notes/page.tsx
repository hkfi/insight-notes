"use client";

import { selectedTagsAtom } from "@/app/_state";
import { NoteCard } from "@/components/NoteCard";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useNotes } from "@/hooks/queries/notes/use-notes";
import { useTags } from "@/hooks/queries/tags/use-tags";
import { useAtom } from "jotai";

type TagBadgeProps = {
  tagId: string;
};
function TagBadge({ tagId }: TagBadgeProps) {
  const [selectedTags, setSelectedTags] = useAtom(selectedTagsAtom);

  const isSelected = selectedTags.includes(tagId);

  return (
    <Badge
      onClick={() => {
        isSelected
          ? setSelectedTags(selectedTags.filter((tag) => tag !== tagId))
          : setSelectedTags([...new Set(selectedTags), tagId]);
      }}
      variant={isSelected ? "default" : "secondary"}
      className="cursor-pointer"
    >
      {tagId}
    </Badge>
  );
}

export default function Page() {
  const [selectedTags, setSelectedTags] = useAtom(selectedTagsAtom);
  const { data: tags } = useTags();

  const { data: notes } = useNotes({
    tag_ids: selectedTags,
  });

  return (
    <div className="flex flex-col gap-2 flex-grow  overflow-auto">
      <div className="flex gap-2 flex-wrap p-2">
        {tags?.map((tag) => (
          <TagBadge key={tag.id} tagId={tag.id} />
        ))}
      </div>
      <ScrollArea>
        <div className="grid grid-cols-1 md:grid-cols-2 p-2 gap-2">
          {notes?.map((note) => (
            <div key={note.id} className="col-span-1">
              <NoteCard className="h-32" note={note} variant="lg" />
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}
