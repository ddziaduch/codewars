<?php declare(strict_types=1);

// https://www.codewars.com/kata/snail/train/php

class Offset
{
    private $x;
    private $y;

    public function __construct(int $xOffset, int $yOffset)
    {
        $this->x = $xOffset;
        $this->y = $yOffset;
    }
    public function getX(): int
    {
        return $this->x;
    }

    public function getY(): int
    {
        return $this->y;
    }
}

abstract class Direction
{
    private $offset;

    public function __construct(int $xOffset, int $yOffset)
    {
        $this->offset = new Offset($xOffset, $yOffset);
    }

    public function getOffset(): Offset
    {
        return $this->offset;
    }

    abstract public function turnRight(): Direction;
}

class North extends Direction
{
    public function __construct()
    {
        parent::__construct(0, -1);
    }

    public function turnRight(): Direction
    {
        return new East();
    }
}

class East extends Direction
{
    public function __construct()
    {
        parent::__construct(1, 0);
    }

    public function turnRight(): Direction
    {
        return new South();
    }
}

class South extends Direction
{
    public function __construct()
    {
        parent::__construct(0, 1);
    }

    public function turnRight(): Direction
    {
        return new West();
    }
}

class West extends Direction
{
    public function __construct()
    {
        parent::__construct(-1, 0);
    }

    public function turnRight(): Direction
    {
        return new North();
    }
}

class Snail
{
    private $visitedCoordinates = [];
    private $direction;
    private $matrix;
    private $currentCoordinates;

    public function __construct(Matrix $matrix)
    {
        $this->matrix = $matrix;
        $this->direction = new East();
        $this->currentCoordinates = new Coordinates(0, 0);
    }

    public function move()
    {
        $this->visitedCoordinates[] = $this->currentCoordinates;
        $candidateForNextCoordinates = $this->getNextCoordinates();

        if (!$this->matrix->containsCoordinates($candidateForNextCoordinates) || in_array($candidateForNextCoordinates, $this->visitedCoordinates)) {
            $this->direction = $this->direction->turnRight();
            $candidateForNextCoordinates = $this->getNextCoordinates();
        }
        $this->currentCoordinates = $candidateForNextCoordinates;
    }

    public function getVisitedCoordinates(): array
    {
        return $this->visitedCoordinates;
    }

    public function canMove()
    {
        return count($this->visitedCoordinates) < $this->matrix->getSize();
    }

    private function getNextCoordinates(): Coordinates
    {
        return $this->currentCoordinates->applyOffset($this->direction->getOffset());
    }
}

class Coordinates
{
    private $x;
    private $y;

    public function __construct(int $x, int $y)
    {
        $this->x = $x;
        $this->y = $y;
    }

    public function getX(): int
    {
        return $this->x;
    }

    public function getY(): int
    {
        return $this->y;
    }

    public function applyOffset(Offset $offset): self
    {
        return new self(
            $this->x + $offset->getX(),
            $this->y + $offset->getY()
        );
    }
}

class Matrix
{
    private $data;

    public function __construct(array $data)
    {
        $this->data = $data;
    }

    public function containsCoordinates(Coordinates $coordinates): bool
    {
        return array_key_exists(
            $coordinates->getY(),
            $this->data
        ) && array_key_exists(
            $coordinates->getX(),
            $this->data[$coordinates->getY()]
        );
    }

    public function getValue(Coordinates $coordinates)
    {
        return $this->containsCoordinates($coordinates) ? $this->data[$coordinates->getY()][$coordinates->getX()] : null;
    }

    public function getSize()
    {
        return count($this->data[0]) ** 2;
    }
}

function snail(array $array): array
{
    $matrix = new Matrix($array);
    $snail = new Snail($matrix);

    while ($snail->canMove()) {
        $snail->move();
    }

    return array_map(function (Coordinates $coordinates) use ($matrix) {
        return $matrix->getValue($coordinates);
    }, $snail->getVisitedCoordinates());
}

var_dump(snail([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]));

