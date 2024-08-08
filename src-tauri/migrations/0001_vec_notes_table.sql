CREATE TABLE `notes` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`content` text DEFAULT '' NOT NULL,
    `average_sentence_embedding` blob,
	`created_at` integer DEFAULT (strftime('%s', 'now')) NOT NULL,
	`updated_at` integer DEFAULT (strftime('%s', 'now'))
);
--> statement-breakpoint
CREATE TABLE `note_chunks` (
    `id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
    `sentence` text NOT NULL,
    `sentence_embedding` blob,
    `note_id` integer,
    FOREIGN KEY(`note_id`) REFERENCES notes(id) ON DELETE CASCADE
);
--> statement-breakpoint
CREATE VIRTUAL TABLE vec_note_chunks using vec0(
	sentence_embedding float[384]
);
--> statement-breakpoint
CREATE TABLE `tags` (
    `id` text NOT NULL UNIQUE PRIMARY KEY
);
--> statement-breakpoint
CREATE TABLE `notes_to_tags` (
    note_id integer NOT NULL,
    tag_id text NOT NULL,
    PRIMARY KEY(note_id, tag_id),
    FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
);