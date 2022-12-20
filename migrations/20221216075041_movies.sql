-- Table: public.movies

-- DROP TABLE IF EXISTS public.movies;

CREATE SEQUENCE movies_id_seq START 1;
CREATE TABLE IF NOT EXISTS public.movies
(
    id integer NOT NULL DEFAULT nextval('movies_id_seq'::regclass),
    title character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT movies_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.movies
    OWNER to postgres;