"use client";

import { Note } from "@/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type CreateNoteParams = { content: string };

async function createNote({ content }: CreateNoteParams) {
  const res = await invoke("create_note", {
    content,
  });

  return res as number;
}

export const useNoteCreate = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: ({ content }: { content: string }) => createNote({ content }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notes"] });
    },
  });

  return mutation;
};
