-- Add migration script here
-- Table: public.movies

-- DROP TABLE IF EXISTS public.movies;

DROP SEQUENCE IF EXISTS movies_id_seq;
CREATE SEQUENCE movies_id_seq START 1;

CREATE TABLE IF NOT EXISTS public.movies
(
    id integer NOT NULL DEFAULT nextval('movies_id_seq'::regclass),
    title character varying COLLATE pg_catalog."default" NOT NULL,
    genres character varying[] COLLATE pg_catalog."default",
    release_year smallint NOT NULL,
    CONSTRAINT movies_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.movies
    OWNER to postgres;