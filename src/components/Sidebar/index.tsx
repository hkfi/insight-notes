"use client";

import { useEffect } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Search } from "../Search";
import { NoteCard } from "../NoteCard";
import { ScrollArea } from "../ui/scroll-area";
import { useTags } from "@/hooks/queries/tags/use-tags";
import { Home } from "lucide-react";
import { NoteTag } from "./NoteTag";
import { useNotes } from "@/hooks/queries/notes/use-notes";
import { useNoteCreate } from "@/hooks/mutations/notes/use-note-create";

export function Sidebar() {
  const router = useRouter();
  const { data: notes } = useNotes({});
  const { data: tags } = useTags();

  const { mutateAsync: createNote } = useNoteCreate();

  const handleCreateNewNote = () => {
    createNote({ content: "New Note" }).then((noteId) => {
      router.push(`/notes/${noteId}`);
    });
  };

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "n" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        handleCreateNewNote();
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        router.push("/");
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  return (
    <div className="h-full w-full flex flex-col justify-between">
      <div className="flex flex-col flex-grow overflow-auto">
        <Button asChild size="sm" variant="link">
          <Link href="/">
            <Home className="w-4 h-4 mr-2" />
            Notes App
          </Link>
        </Button>

        <Search />

        <ScrollArea>
          <div className="flex flex-col gap-1 p-1">
            {(notes ?? []).map((note, i) => (
              <NoteCard key={i} note={note} />
            ))}
          </div>
        </ScrollArea>

        <div className="text-sm text-center">Tags</div>
        <ScrollArea>
          <div className="flex gap-1 p-1 flex-wrap">
            {tags?.map((tag) => (
              <NoteTag key={tag.id} tagId={tag.id} />
            ))}
          </div>
        </ScrollArea>
      </div>
      <Button
        className="flex w-full justify-between gap-2"
        variant="ghost"
        size="sm"
        onClick={() => {
          handleCreateNewNote();
        }}
      >
        <span className="text-sm text-muted-foreground">New Note</span>
        <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
          <span className="text-xs">⌘</span>N
        </kbd>
      </Button>
    </div>
  );
}
