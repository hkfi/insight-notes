"use client";

import { Tag } from "@/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/tauri";

export const useTags = () => {
  const query = useQuery({
    queryKey: ["tags"],
    queryFn: async () => {
      const tags = await invoke("get_tags");
      return tags as Tag[];
    },
  });

  return query;
};
