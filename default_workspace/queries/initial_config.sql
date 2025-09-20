BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "providers" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	"url"	TEXT NOT NULL,
	"api_key"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "storage" (
	"id"	INTEGER,
	"key"	TEXT NOT NULL UNIQUE,
	"value"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
INSERT INTO "providers" VALUES (1,'Ollama Native','http://127.0.0.1:11434/v1','0');
INSERT INTO "storage" VALUES (1,'current_provider_id','1');
COMMIT;
