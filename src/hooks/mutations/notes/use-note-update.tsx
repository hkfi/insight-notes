"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type UpdateNoteParams = { id: number; content: string };

async function updateNote({ id, content }: UpdateNoteParams) {
  const res = await invoke("update_note", {
    id: Number(id),
    content,
  });

  return res;
}

export const useNoteUpdate = ({ id }: { id: number }) => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (content: string) => updateNote({ id, content }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["note", id] });
      queryClient.invalidateQueries({ queryKey: ["relatedWords", id] });
      queryClient.invalidateQueries({ queryKey: ["notes"] });
    },
  });

  return mutation;
};
