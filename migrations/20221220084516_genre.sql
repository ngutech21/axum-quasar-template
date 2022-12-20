-- Add migration script here
-- Table: public.genres

-- DROP TABLE IF EXISTS public.genres;

CREATE SEQUENCE genre_id_seq START 1;
CREATE TABLE IF NOT EXISTS public.genres
(
    id integer NOT NULL DEFAULT nextval('genre_id_seq'::regclass),
    name character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT genre_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.genres
    OWNER to postgres;


-- Table: public.genres_to_movies

-- DROP TABLE IF EXISTS public.genres_to_movies;

CREATE SEQUENCE genres_to_movies_id_seq START 1;
CREATE TABLE IF NOT EXISTS public.genres_to_movies
(
    id integer NOT NULL DEFAULT nextval('genres_to_movies_id_seq'::regclass),
    genre_id bigint NOT NULL,
    movie_id bigint NOT NULL,
    CONSTRAINT genres_to_movies_pkey PRIMARY KEY (id),
    CONSTRAINT fk_genre FOREIGN KEY (genre_id)
        REFERENCES public.genres (id) MATCH SIMPLE
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
        NOT VALID,
    CONSTRAINT fk_movie FOREIGN KEY (movie_id)
        REFERENCES public.movies (id) MATCH SIMPLE
        ON UPDATE RESTRICT
        ON DELETE RESTRICT
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.genres_to_movies
    OWNER to postgres;