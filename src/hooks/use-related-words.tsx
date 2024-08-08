"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { useQuery } from "@tanstack/react-query";

export const useRelatedWords = ({ noteId }: { noteId: number }) => {
  const query = useQuery({
    queryKey: ["relatedWords", noteId],
    queryFn: async () => {
      const relatedWords = await invoke("get_similar_words", { noteId });
      return relatedWords as string[];
    },
  });

  return query;
};
