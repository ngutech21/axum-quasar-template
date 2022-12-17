-- Table: public.movies

-- DROP TABLE IF EXISTS public.movies;

CREATE TABLE IF NOT EXISTS public.movies
(
    id bigint NOT NULL,
    title character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT movies_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.movies
    OWNER to axum;