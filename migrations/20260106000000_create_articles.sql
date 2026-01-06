-- Create articles table
CREATE TABLE IF NOT EXISTS articles (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	hn_id INTEGER UNIQUE NOT NULL,
	title TEXT NOT NULL,
	url TEXT,
	score INTEGER NOT NULL DEFAULT 0,
	timestamp INTEGER NOT NULL,
	fetched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	ai_analysis_done BOOLEAN NOT NULL DEFAULT 0,
	is_interesting BOOLEAN NOT NULL DEFAULT 0,
	reason TEXT,
	priority INTEGER DEFAULT NULL,
	UNIQUE(hn_id)
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_is_interesting ON articles(is_interesting, fetched_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_analysis_done ON articles(ai_analysis_done);
CREATE INDEX IF NOT EXISTS idx_fetched_at ON articles(fetched_at DESC);
