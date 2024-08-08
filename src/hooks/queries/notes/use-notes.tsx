"use client";

import { Note } from "@/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

type SearchParams = {
  tag_ids?: string[];
  match_all?: boolean;
  skip?: number;
  take?: number;
};

export const useNotes = ({
  tag_ids = [],
  match_all = false,
  skip = 0,
  take = 50,
}: SearchParams) => {
  const query = useQuery({
    queryKey: ["notes", tag_ids, match_all, skip, take],
    queryFn: async () => {
      const notes = await invoke("get_notes", {
        params: {
          tag_ids,
          match_all,
          skip,
          take,
        },
      });
      return notes as Note[];
    },
  });

  return query;
};
