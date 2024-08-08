"use client";

import { useEffect, useState } from "react";
import { useDebounce } from "use-debounce";

import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";

import type { Note } from "@/types";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { atom, useAtom } from "jotai";
import { useNotesSearch } from "@/hooks/queries/notes/use-notes-search";

const searchStringAtom = atom("");
const searchResultsAtom = atom<Note[]>([]);

export function Search() {
  const [open, setOpen] = useState(false);
  const router = useRouter();

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "f" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((open) => !open);
      }
    };

    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  const [searchString, setSearchString] = useAtom(searchStringAtom);
  const [query] = useDebounce(searchString, 200);

  const { data: searchResults } = useNotesSearch({ searchTerm: query });

  return (
    <>
      <Button
        className="flex w-full justify-between gap-2"
        variant="ghost"
        size="sm"
        onClick={() => {
          setOpen(true);
        }}
      >
        <span className="text-sm text-muted-foreground">Search...</span>
        <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
          <span className="text-xs">⌘</span>F
        </kbd>
      </Button>
      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput
          placeholder="Search..."
          value={searchString}
          onValueChange={(s) => setSearchString(s)}
        />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          {(searchResults?.length ?? 0) > 0 && (
            <CommandGroup heading="Search">
              {(searchResults ?? []).map((note, i) => {
                return (
                  <CommandItem
                    className="overflow-hidden text-ellipsis line-clamp-1"
                    key={i}
                    onSelect={() => {
                      setOpen(false);
                      router.push(`/notes/${note.id}`);
                    }}
                  >
                    <span>{note.content.slice(0, 100)}</span>
                  </CommandItem>
                );
              })}
            </CommandGroup>
          )}
        </CommandList>
      </CommandDialog>
    </>
  );
}
