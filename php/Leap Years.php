<?php

declare(strict_types=1);

// https://www.codewars.com/kata/leap-years/train/php

function isLeapYear(int $year): bool {
    return $year % 400 === 0 || $year % 4 === 0 && $year % 100 !== 0;
}

assert(true === isLeapYear(2016));
assert(true === isLeapYear(2000));
assert(false === isLeapYear(2100));
assert(false === isLeapYear(1900));