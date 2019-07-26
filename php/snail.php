<?php declare(strict_types=1);

// https://www.codewars.com/kata/snail/train/php

function snail(array $array): array
{
    $results = [];
    $x = 0;
    $y = 0;
    $i = 0;
    $n = count($array);
    $direction = 'left';

    while ($i < $n * $n) {
        if ($i > 4) break;
        if (!in_array([$x, $y], $results)) {
            $results[] = [$x, $y];
        }

        $newX = $x;
        $newY = $y;

        if ($direction === 'left') {
            $newX = $x + 1;
        } elseif ($direction === 'down') {
            $newY = $y + 1;
        } elseif ($direction === 'right') {
            $newX = $x - 1;
        } elseif ($direction === 'up') {
            $newY = $y - 1;
        }

        if ($newX >= $n && $direction === 'left') {
            $direction = 'down';
        } else {
            $x = $newX;
            $y = $newY;
            $i++;
        }
    }

    var_dump($results);

    return $results;
}

snail([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]);

