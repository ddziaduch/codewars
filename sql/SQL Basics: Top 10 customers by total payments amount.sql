-- https://www.codewars.com/kata/580d08b5c049aef8f900007c/train/sql

SELECT customer.customer_id,
       customer.email,
       count(payment.payment_id) AS payments_count,
       cast(sum(payment.amount) AS FLOAT) AS total_amount
FROM customer
         JOIN payment ON payment.customer_id = customer.customer_id
GROUP BY customer.customer_id
ORDER BY total_amount DESC
LIMIT 10
