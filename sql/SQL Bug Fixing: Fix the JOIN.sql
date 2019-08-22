-- https://www.codewars.com/kata/580fb94e12b34dd1c40001f0/train/sql
SELECT job.job_title,
       ROUND(SUM(job.salary) / COUNT(people.id), 2)::FLOAT AS average_salary,
       COUNT(people.id) AS total_people,
       ROUND(SUM(job.salary), 2)::FLOAT AS total_salary
FROM people
         JOIN job ON job.people_id = people.id
GROUP BY job.job_title
ORDER BY average_salary DESC