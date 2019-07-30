<?php declare(strict_types=1);

// https://www.codewars.com/kata/snail/train/php

abstract class Direction
{
    private $xOffset;
    private $yOffset;

    public function __construct(int $xOffset, int $yOffset)
    {
        $this->xOffset = $xOffset;
        $this->yOffset = $yOffset;
    }
    public function getXOffset(): int
    {
        return $this->xOffset;
    }

    public function getYOffset(): int
    {
        return $this->yOffset;
    }
    public function isEqualTo(Direction $other)
    {
        return $this->xOffset === $other->xOffset && $this->yOffset === $other->yOffset;
    }
}

class Up extends Direction
{
    public function __construct()
    {
        parent::__construct(0, -1);
    }
}

class Right extends Direction
{
    public function __construct()
    {
        parent::__construct(1, 0);
    }
}

class Down extends Direction
{
    public function __construct()
    {
        parent::__construct(0, 1);
    }
}

class Left extends Direction
{
    public function __construct()
    {
        parent::__construct(-1, 0);
    }
}

function snail(array $array): array
{
    $up = new Up();
    $right = new Right();
    $down = new Down();
    $left = new Left();

    $coordinates = [];
    $x = 0;
    $y = 0;
    $i = 0;
    $n = count($array[0]);
    $numberOfElements = $n * $n;

    $direction = $right;

    while ($i < $numberOfElements) {
        $currentElementKey = "$x:$y";
        $coordinates[] = $currentElementKey;
        $nextElementKey = sprintf("%s:%s", $x + $direction->getXOffset(), $y + $direction->getYOffset());

        if (!in_array($nextElementKey, $coordinates)) {
            if ($x + $direction->getXOffset() >= $n) {
                $direction = $down;
            } elseif ($y + $direction->getYOffset() >= $n) {
                $direction = $left;
            } elseif ($x + $direction->getXOffset() < 0) {
                $direction = $up;
            }
        } else {
            if ($direction instanceof Up) {
                $direction = $right;
            } elseif ($direction instanceof Right) {
                $direction = $down;
            } elseif ($direction instanceof Down) {
                $direction = $left;
            } elseif ($direction instanceof Left) {
                $direction = $up;
            }
        }
        $x = $x + $direction->getXOffset();
        $y = $y + $direction->getYOffset();
        $i++;
    }

    $results = array_map(function ($point) use ($array) {
        list($x, $y) = explode(':', $point);
        return $array[$y][$x];
    } , $coordinates);

    return $results;
}

var_dump(snail([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]));

