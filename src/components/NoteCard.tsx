import { format } from "date-fns";

import type { Note } from "@/types";
import { useParams } from "next/navigation";
import Link from "next/link";
import { cn, extractFirstLineOfString } from "@/lib/utils";
import { formatDateShort, formatTimeAgo } from "@/lib/time";

type NoteCardProps = {
  note: Note;
  variant?: "sm" | "lg";
  className?: string;
};

export function NoteCard({ note, variant = "sm", className }: NoteCardProps) {
  const { id } = useParams();

  return (
    <Link
      className={cn(
        `flex flex-col justify-between rounded-lg border border-accent whitespace-nowrap overflow-ellipsis overflow-hidden ${
          Number(id) === note?.id ? "bg-accent" : ""
        } hover:bg-accent p-1 ${variant === "sm" ? "h-12" : ""}`,
        className
      )}
      href={`/notes/${note.id}`}
    >
      {note.content.length === 0 ? (
        <span className="text-sm text-muted-foreground italic">Empty</span>
      ) : (
        <span className="text-sm overflow-hidden text-ellipsis whitespace-pre-wrap">
          {variant === "sm"
            ? extractFirstLineOfString(note.content).slice(0, 100)
            : note.content.slice(0, 200)}
        </span>
      )}
      <span className="text-xs text-gray-600 text-end">
        {note.updated_at ? formatTimeAgo(note.updated_at) : null}
      </span>
    </Link>
  );
}
