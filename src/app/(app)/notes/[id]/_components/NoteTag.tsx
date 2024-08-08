import { selectedTagsAtom } from "@/app/_state";
import { Badge } from "@/components/ui/badge";
import { useNoteTagDelete } from "@/hooks/mutations/notes/use-note-tag-delete";
import { useAtom } from "jotai";
import { Search, X } from "lucide-react";
import { useRouter } from "next/navigation";

type NoteTagProps = {
  noteId: number;
  tagId: string;
};

export function NoteTag({ noteId, tagId }: NoteTagProps) {
  const router = useRouter();
  const [, setSelectedTags] = useAtom(selectedTagsAtom);

  const { mutateAsync: deleteNoteTag } = useNoteTagDelete({ noteId });

  return (
    <Badge
      variant="outline"
      className="cursor-pointer group p-0 overflow-hidden"
    >
      <div
        onClick={() => {
          setSelectedTags([tagId]);
          router.push(`/notes`);
        }}
        className="gap-1 flex items-center p-1 hover:bg-muted"
      >
        <Search className="invisible w-3 h-3 group-hover:visible" />
        {tagId}
      </div>
      <div className="p-1 h-full flex items-center hover:bg-muted invisible group-hover:visible">
        <X
          onClick={async () => {
            await deleteNoteTag({ tagId });
          }}
          className="w-3 h-3"
        />
      </div>
    </Badge>
  );
}
