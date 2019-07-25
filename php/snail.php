<?php declare(strict_types=1);

// https://www.codewars.com/kata/snail/train/php

function snail(array $array): array
{
    $results = [];
    $xOffset = 1;
    $yOffset = 0;
    $n = count($array);
    $x = 0;
    $y = 0;
    $xToWalk = 3;
    $yToWalk = 0;
    $i = 0;

    while ($i < 4) {
        $results[] = $array[$y][$x];
        if ($xOffset !== 0) {
            if ($xToWalk > 0) {
                $xToWalk -= 1;
                $x += $xOffset;
            }
        }
        $i++;
    }

    var_dump($results);

    return $results;
}

snail([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]);

