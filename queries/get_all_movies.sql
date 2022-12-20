select m.id as movie_id, m.title as movie_title, 
g.id as genre_id, g.name as genre_name
from movies m 
LEFT JOIN genres_to_movies piv on m.id = piv.movie_id 
LEFT JOIN genres g on g.id = piv.genre_id 
order by m.id
