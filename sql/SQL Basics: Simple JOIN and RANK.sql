SELECT people.*,
       count(sales.id) AS sale_count,
       rank() OVER (ORDER BY count(sales.id) DESC) AS sale_rank
FROM people
         JOIN sales ON sales.people_id = people.id
GROUP BY people.id
ORDER BY sale_count DESC