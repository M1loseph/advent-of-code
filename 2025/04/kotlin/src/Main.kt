import java.nio.file.Files
import java.nio.file.Path

enum class GridElement(val value: Char) {
    ROLL_OF_PAPER('@'),
    EMPTY('.');
}

data class Point(val x: Int, val y: Int) {
    fun isValid(maxX: Int, maxY: Int): Boolean {
        if (x < 0 || y < 0) {
            return false
        }
        if (x > maxX || y > maxY) {
            return false
        }
        return true
    }
}

typealias Grid = List<List<GridElement>>

fun Grid.printPretty() {
    forEach { y ->
        y.forEach {
            print(it.value)
        }
        println()
    }
}

fun Grid.mutable(): MutableList<MutableList<GridElement>> {
    return map { it.toMutableList() }.toMutableList()
}

fun puzzle1(grid: Grid) {
    var rollsOfPaper = 0
    for (y in grid.indices) {
        val maxY = grid.size - 1
        for (x in grid[y].indices) {
            if (grid[y][x] != GridElement.ROLL_OF_PAPER) {
                continue
            }
            val maxX = grid[y].size - 1
            val roolsOfPaperNearby = listOf(
                Point(x - 1, y - 1),
                Point(x - 1, y),
                Point(x - 1, y + 1),
                Point(x, y - 1),
                Point(x, y + 1),
                Point(x + 1, y - 1),
                Point(x + 1, y),
                Point(x + 1, y + 1),
            )
                .filter { it.isValid(maxX, maxY) }
                .count { grid[it.y][it.x] == GridElement.ROLL_OF_PAPER }
            if (roolsOfPaperNearby < 4) {
                rollsOfPaper += 1
            }
        }
    }
    println("Puzzle 1 solution: $rollsOfPaper")
}

fun puzzle2(grid: Grid) {
    val grid = grid.mutable()
    var totalRollsOfPaper = 0
    while (true) {
        var rollsOfPaper = 0
        for (y in grid.indices) {
            val maxY = grid.size - 1
            for (x in grid[y].indices) {
                if (grid[y][x] != GridElement.ROLL_OF_PAPER) {
                    continue
                }
                val maxX = grid[y].size - 1
                val roolsOfPaperNearby = listOf(
                    Point(x - 1, y - 1),
                    Point(x - 1, y),
                    Point(x - 1, y + 1),
                    Point(x, y - 1),
                    Point(x, y + 1),
                    Point(x + 1, y - 1),
                    Point(x + 1, y),
                    Point(x + 1, y + 1),
                )
                    .filter { it.isValid(maxX, maxY) }
                    .count { grid[it.y][it.x] == GridElement.ROLL_OF_PAPER }
                if (roolsOfPaperNearby < 4) {
                    grid[y][x] = GridElement.EMPTY
                    rollsOfPaper += 1
                }
            }
        }
        if (rollsOfPaper == 0) {
            break
        }
        totalRollsOfPaper += rollsOfPaper
    }
    println("Puzzle 2 solution: $totalRollsOfPaper")
}

fun main() {
    val input = Files.readString(Path.of("input.txt"))
    val grid = input.lines()
        .map {
            it.toCharArray().map { char ->
                when (char) {
                    '@' -> GridElement.ROLL_OF_PAPER
                    '.' -> GridElement.EMPTY
                    else -> throw IllegalArgumentException("Unknown char $char")
                }
            }
        }
    grid.printPretty()
    puzzle1(grid)
    puzzle2(grid)
}