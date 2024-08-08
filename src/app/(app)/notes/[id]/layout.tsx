"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { ReactNode } from "react";
import { RelatedNotes } from "./_components/RelatedNotes";
import { RelatedWords } from "./_components/RelatedWords";

export default function Layout({
  children,
  params: { id },
}: {
  children: ReactNode;
  params: {
    id: string;
  };
}) {
  return (
    <ResizablePanelGroup direction="horizontal" className="flex flex-grow">
      <ResizablePanel className="flex-grow flex flex-col">
        {children}
      </ResizablePanel>

      <ResizableHandle />

      <ResizablePanel className="flex flex-col" minSize={15} defaultSize={15}>
        <RelatedWords noteId={Number(id)} />
        <RelatedNotes noteId={Number(id)} />
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}
