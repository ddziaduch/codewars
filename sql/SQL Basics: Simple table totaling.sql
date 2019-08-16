SELECT RANK() OVER (ORDER BY sum(points) DESC),
       CASE WHEN clan = '' THEN '[no clan specified]' ELSE clan END,
       sum(points) AS total_points,
       count(id)   AS total_people
FROM people
GROUP BY clan
ORDER BY total_points DESC