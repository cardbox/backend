-- Add down migration script here
DROP INDEX socials_user_id;

CREATE UNIQUE INDEX "socials_user_id" ON socials USING btree ("user_id");