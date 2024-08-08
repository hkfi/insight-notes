"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type DeleteNoteParams = { id: number };

async function deleteNote({ id }: DeleteNoteParams) {
  const res = await invoke("delete_note", {
    id: Number(id),
  });

  return res;
}

export const useNoteDelete = ({ id }: { id: number }) => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: () => deleteNote({ id }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notes"] });
    },
  });

  return mutation;
};
