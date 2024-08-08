"use client";

import { Note } from "@/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

export const useNote = ({ id }: { id: number }) => {
  const query = useQuery({
    queryKey: ["note", id],
    queryFn: async () => {
      const note = await invoke("get_note", {
        id,
      });
      return note as Note;
    },
  });

  return query;
};
