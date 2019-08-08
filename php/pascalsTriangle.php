<?php declare(strict_types=1);

// https://www.codewars.com/kata/5226eb40316b56c8d500030f/train/php

function pascals_triangle($n) {
    return pascals_triangle_1d($n);
}

function pascals_triangle_1d(int $depth): array
{
    $triangle2d = pascal_triangle_2d($depth);
    $results = [];
    foreach ($triangle2d as $row) {
        foreach ($row as $value) {
            $results[] = $value;
        }
    }
    return $results;
}

function pascal_triangle_2d(int $depth): array
{
    $triangle2d = [];
    for ($level = 0; $level < $depth; $level += 1) {
        $triangle2d[$level] = [];
        for ($position = 0; $position <= $level; $position += 1) {
            if ($position === 0) {
                $triangle2d[$level][$position] = 1;
            } elseif ($position === $level) {
                $triangle2d[$level][$position] = 1;
            } else {
                $triangle2d[$level][$position] = $triangle2d[$level - 1][$position - 1] + $triangle2d[$level - 1][$position];
            }
        }
    }
    return $triangle2d;
}

assert([[1]] === pascal_triangle_2d(1));
assert([[1], [1, 1]] === pascal_triangle_2d(2));
assert([[1], [1, 1], [1, 2, 1]] === pascal_triangle_2d(3));
assert([[1], [1, 1], [1, 2, 1], [1, 3, 3, 1]] === pascal_triangle_2d(4));
assert(
    [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]] ===
    pascal_triangle_2d(5)
);

assert([1] === pascals_triangle_1d(1));
assert([1, 1, 1] === pascals_triangle_1d(2));
assert([1, 1, 1, 1, 2, 1] === pascals_triangle_1d(3));
assert([1, 1, 1, 1, 2, 1, 1, 3, 3, 1] === pascals_triangle_1d(4));
assert(
    [1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1] ===
    pascals_triangle_1d(5)
);
