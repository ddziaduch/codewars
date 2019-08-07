<?php declare(strict_types=1);

// https://www.codewars.com/kata/520446778469526ec0000001/train/php

function same_structure_as(array $a, array $b): bool
{
    $has_a_array = has_array_an_array_inside($a);
    $has_b_array = has_array_an_array_inside($b);
    if ($has_a_array !== $has_b_array) {
        return false;
    }
    if (!$has_a_array && !$has_b_array) {
        return count($a) === count($b);
    }
    return same_structure_as_when_both_arrays_have_arrays_inside($a, $b);
}

function same_structure_as_when_both_arrays_have_arrays_inside(array $a, array $b): bool
{
    foreach ($a as $k => $v) {
        if (is_array($v) && !is_array($b[$k])) {
            return false;
        }
        if (!is_array($v) && is_array($b[$k])) {
            return false;
        }
        if (is_array($v) && is_array($b[$k]) && !same_structure_as($v, $b[$k])) {
            return false;
        }
    }
    return true;
}

function has_array_an_array_inside($a): bool
{
    foreach ($a as $v) {
        if (is_array($v)) {
            return true;
        }
    }
    return false;
}

assert(same_structure_as([1, 1, 1], [2, 2, 2]) === true);
assert(same_structure_as([1, [1, 1]], [2, [2, 2]]) === true);
assert(same_structure_as([1, [1, 1]], [[2, 2], 2]) === false);
assert(same_structure_as([1, [1, 1]], [[2], 2]) === false);
assert(same_structure_as([[[], []]], [[[], []]]) === true);
assert(same_structure_as([[[], []]], [[1, 1]]) === false);