"use client";

import { useMemo } from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { NoteCard } from "@/components/NoteCard";
import { useNotes } from "@/hooks/queries/notes/use-notes";

export default function Page() {
  const { data: notes } = useNotes({});

  const tenMostRecentNotes = useMemo(() => {
    return notes?.slice(0, 10) ?? [];
  }, [notes]);

  return (
    <div className="flex flex-col overflow-auto flex-grow">
      <ScrollArea>
        <div className="grid grid-cols-1 md:grid-cols-2 p-2 gap-2">
          {tenMostRecentNotes?.map((note) => (
            <div key={note.id} className="col-span-1">
              <NoteCard className="h-32" note={note} variant="lg" />
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}
