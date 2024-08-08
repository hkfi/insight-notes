"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type DeleteTagParams = { id: string };

async function deleteTag({ id }: DeleteTagParams) {
  const res = await invoke("delete_tag", {
    id,
  });

  return res;
}

export const useTagDelete = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: ({ id }: { id: string }) => deleteTag({ id }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notes"] });
      queryClient.invalidateQueries({ queryKey: ["tags"] });
    },
  });

  return mutation;
};
