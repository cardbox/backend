CREATE TABLE "session_tokens" (
  "user_id" uuid NOT NULL REFERENCES users(id),
  "token" varchar NOT NULL,
  "expires_at" timestamp NOT NULL,
  PRIMARY KEY ("token")
);

CREATE INDEX "session_tokens_user_id" ON "session_tokens" USING BTREE ("user_id");
CREATE INDEX "session_tokens_expires_at" ON "session_tokens" USING BTREE ("expires_at");
