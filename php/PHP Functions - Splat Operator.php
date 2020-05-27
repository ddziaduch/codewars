<?php declare(strict_types=1);

// https://www.codewars.com/kata/57d3c6a043942a34d200015e/train/php

function sum_all(...$numbers) {
    return array_sum($numbers);
}

function ultimate_product(...$numbers) {
    return array_product($numbers);
}

function scale_sum($x, ...$numbers) {
    return sum_all(...$numbers) * $x;
}

assert(15 === sum_all(1, 2, 3, 4, 5));
assert(953 === sum_all(13, 938, 2));
assert(11102 === sum_all(198, 283, 9826, 2, 4, 789));
assert(1 === ultimate_product(1));
assert(366973740 === ultimate_product(54, 23, 45, 67, 98));
assert(3628800 === ultimate_product(...array(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)));
assert(70 === scale_sum(5, 2, 3, 4, 5));
assert(13013 === scale_sum(13, 68, 5, 798, 56, 65, 9));
assert(64 === scale_sum(4, ...array(4, 4, 4, 4)));
