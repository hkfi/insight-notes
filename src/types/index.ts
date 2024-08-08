export type Note = {
  id: number;
  content: string;
  // seconds since Unix epoch.
  // for js, it needs to be milliseconds
  created_at: number;
  updated_at: number;
  tags: Tag[];
};

export type Tag = {
  id: string;
};
