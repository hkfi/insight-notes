"use client";

import { useCallback, useEffect, useState } from "react";
import { MdEditor } from "md-editor-rt";
import "md-editor-rt/lib/style.css";
import { useDebounce } from "use-debounce";
import type { Note } from "@/types";
import { LoadingSpinner } from "@/components/ui/loading-spinner";
import { Check, Trash } from "lucide-react";
import { useRouter } from "next/navigation";
import { invoke } from "@tauri-apps/api/tauri";

import { NoteTag } from "./_components/NoteTag";
import { formatDate } from "@/lib/time";
import { useNoteUpdate } from "@/hooks/mutations/notes/use-note-update";
import { useNote } from "@/hooks/queries/notes/use-note";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

function NoteIsUpdatingSpinner({ loading }: { loading: boolean }) {
  return (
    <div className="flex items-center justify-center p-1">
      {loading ? (
        <LoadingSpinner className="h-4 w-4" />
      ) : (
        <Check className="h-4 w-4" />
      )}
    </div>
  );
}

export default function Page({ params }: { params: { id: string } }) {
  const id = Number(params.id);
  const { data: note, isLoading } = useNote({ id });
  const { mutate: updateNote, isPending } = useNoteUpdate({ id });
  const router = useRouter();
  const [touched, setTouched] = useState(false);
  const [noteContent, setNoteContent] = useState<undefined | string>("");

  const [noteContentVal] = useDebounce(noteContent, 500);

  const handleSave = useCallback(() => {
    if (!noteContentVal || isLoading || !touched) return;
    console.log("saving");

    updateNote(noteContentVal);
  }, [noteContentVal, isLoading, touched, updateNote]);

  useEffect(() => {
    handleSave();
  }, [noteContentVal]);

  useEffect(() => {
    invoke("get_note", { id }).then((note) => {
      setNoteContent((note as Note).content);
    });
    setTouched(false);
  }, []);

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "s" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        handleSave();
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, [handleSave]);

  return (
    <div className="flex flex-grow">
      <div
        data-color-mode="light"
        className="flex flex-col flex-grow h-full relative"
      >
        <div className="flex justify-between p-1 items-center">
          <div className="p-1 text-sm">
            {note ? formatDate(note.updated_at) : null}
          </div>
          <div className="flex gap-1 items-center">
            <div className="flex gap-1">
              {note?.tags.map((tag) => (
                <NoteTag key={tag.id} tagId={tag.id} noteId={id} />
              ))}
            </div>

            <Dialog>
              <DialogTrigger asChild>
                <Button variant="ghost" size="sm">
                  <Trash className="w-4 h-4" />
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Delete note</DialogTitle>
                  <DialogDescription>
                    This note will be permenantly deleted
                  </DialogDescription>
                  <DialogFooter>
                    <Button
                      variant="destructive"
                      onClick={() => {
                        invoke("delete_note", { id });
                        router.push(`/`);
                      }}
                    >
                      Delete
                    </Button>
                  </DialogFooter>
                </DialogHeader>
              </DialogContent>
            </Dialog>
          </div>
        </div>
        <MdEditor
          toolbarsExclude={[
            "sub",
            "sup",
            "image",
            "mermaid",
            "save",
            "github",
            "htmlPreview",
            "previewOnly",
            "catalog",
          ]}
          defFooters={[
            "markdownTotal",
            <NoteIsUpdatingSpinner key={id} loading={isPending} />,
          ]}
          footers={["markdownTotal", "=", 1]}
          language="en-US"
          modelValue={noteContent ?? ""}
          className="flex-grow"
          onChange={(val) => {
            if (!touched) {
              setTouched(true);
            }
            setNoteContent(val);
          }}
          autoFocus
        />
      </div>
    </div>
  );
}
