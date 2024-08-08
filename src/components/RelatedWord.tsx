import { Badge } from "./ui/badge";
import { Plus } from "lucide-react";
import { useTagCreate } from "@/hooks/mutations/tags/use-tag-create";

type TagProps = {
  noteId: number;
  word: string;
};

export function RelatedWord({ noteId, word }: TagProps) {
  const { mutate: createTag } = useTagCreate({
    noteId,
  });

  return (
    <Badge
      onClick={() => {
        createTag({ word });
      }}
      className="hover:bg-accent cursor-pointer gap-1"
      variant={"outline"}
    >
      <Plus className="w-3 h-3" />
      {word}
    </Badge>
  );
}
