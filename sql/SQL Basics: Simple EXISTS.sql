-- https://www.codewars.com/kata/58113a64e10b53ec36000293/train/sql

SELECT *
FROM departments
WHERE EXISTS(
  SELECT department_id sum
  FROM sales
  WHERE price > 98
    AND sales.department_id = departments.id
  GROUP BY department_id
  ORDER BY department_id
)