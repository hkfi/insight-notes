"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type CreateTagParams = { noteId: number; word: string };

async function createTag({ noteId, word }: CreateTagParams) {
  const res = await invoke("create_tag", {
    name: word,
    noteId,
  });

  return res;
}

export const useTagCreate = ({ noteId }: { noteId: number }) => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: ({ word }: { word: string }) => createTag({ noteId, word }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["note", noteId] });
    },
  });

  return mutation;
};
