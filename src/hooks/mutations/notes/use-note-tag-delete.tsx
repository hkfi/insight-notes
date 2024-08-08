"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type DeleteNoteTagParams = { tagId: string; noteId: number };

async function deleteNoteTag({ tagId, noteId }: DeleteNoteTagParams) {
  const res = await invoke("delete_note_tag", {
    noteId,
    tagId,
  });

  return res;
}

export const useNoteTagDelete = ({ noteId }: { noteId: number }) => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: ({ tagId }: { tagId: string }) =>
      deleteNoteTag({ noteId, tagId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notes"] });
      queryClient.invalidateQueries({ queryKey: ["note", noteId] });
    },
  });

  return mutation;
};
