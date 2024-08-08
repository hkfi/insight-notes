"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import type { Note } from "@/types";
import { listen } from "@tauri-apps/api/event";

export const useRelatedNotes = ({ noteId }: { noteId: number }) => {
  const [relatedNotes, setRelatedNotes] = useState<Note[]>([]);
  useEffect(() => {
    invoke("find_similar_notes", { noteId }).then((notes) => {
      console.log("find_similar_notes: ", notes);
      setRelatedNotes(notes as Note[]);
    });
  }, []);

  useEffect(() => {
    const unlisten = listen("refetch_notes", (payload) => {
      invoke("find_similar_notes", { noteId }).then((res) => {
        setRelatedNotes(res as Note[]);
      });
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return {
    relatedNotes,
  };
};
