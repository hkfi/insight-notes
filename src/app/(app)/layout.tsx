"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Sidebar } from "@/components/Sidebar";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <ResizablePanelGroup direction="horizontal" className="flex !h-screen">
      <ResizablePanel minSize={15} defaultSize={15} maxSize={30}>
        <Sidebar />
      </ResizablePanel>

      <ResizableHandle />

      <ResizablePanel className="flex-col flex">{children}</ResizablePanel>
    </ResizablePanelGroup>
  );
}
