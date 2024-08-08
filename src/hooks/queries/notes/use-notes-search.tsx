"use client";

import { Note } from "@/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type SearchParams = {
  searchTerm: string;
};

export const useNotesSearch = ({ searchTerm }: SearchParams) => {
  const query = useQuery({
    queryKey: ["notes", searchTerm],
    queryFn: async () => {
      const notes = await invoke("search_notes", {
        query: searchTerm,
      });
      return notes as Note[];
    },
  });

  return query;
};
