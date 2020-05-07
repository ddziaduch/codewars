<?php // https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/php

declare(strict_types=1);

function duplicateCount(string $text): int
{
    return array_reduce(
        count_chars(strtolower($text)),
        static function (int $result, int $charCount) {
            return $charCount > 1 ? $result + 1 : $result;
        },
        0
    );
}

assert(0 === duplicateCount(""));
assert(0 === duplicateCount("abcde"));
assert(2 === duplicateCount("aabbcde"));
assert(2 === duplicateCount("aabBcde"), "should ignore case");
assert(1 === duplicateCount("Indivisibility"));
assert(2 === duplicateCount("Indivisibilities"), "characters may not be adjacent");
