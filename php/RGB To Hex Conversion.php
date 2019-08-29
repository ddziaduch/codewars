<?php

declare(strict_types=1);

function rgb(int $r, int $g, int $b): string
{
    return implode('', array_map(static function (int $n): string {
        return sprintf('%02X', min(255, max(0, $n)));
    }, [$r, $g, $b]));
}

assert('FFFFFF' === rgb(255, 255, 255));
assert("FFFFFF" === rgb(255, 255, 300));
assert("000000" === rgb(0,0,0));
assert("000000" === rgb(-500,0,0));
assert("9400D3" === rgb(148, 0, 211));