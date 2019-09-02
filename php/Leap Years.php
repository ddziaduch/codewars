<?php

declare(strict_types=1);

// https://www.codewars.com/kata/leap-years/train/php

function isLeapYear(int $year): bool {
    if ($year % 4 === 0) {
        if ($year % 100 === 0) {
            return $year % 400 === 0;
        }
        return true;
    }
    return false;
}

assert(true === isLeapYear(2016));
assert(true === isLeapYear(2000));
assert(false === isLeapYear(2100));
assert(false === isLeapYear(1900));