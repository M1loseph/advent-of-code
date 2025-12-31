import java.nio.file.Files
import java.nio.file.Path

data class Machine(
    val lights: List<Boolean>,
    val buttons: List<List<Int>>,
    val joltageRequirements: List<Int>,
)

fun puzzle1(machines: List<Machine>) {
    val puzzle1 = machines.sumOf { solveMachineIndicatorLights(it) }
    println("Puzzle 1 result: $puzzle1")
}

fun solveMachineIndicatorLights(machine: Machine): Int {
    val expected = machine.lights
    val visited = mutableMapOf(expected.map { false } to 0)
    val queue = mutableListOf(expected.map { false })
    while (true) {
        val current = queue.removeFirst()
        val depth = visited[current]!!
        machine.buttons.forEach { button ->
            val nextDepth = depth + 1
            val nextState = current.mapIndexed { index, light ->
                if (button.contains(index)) {
                    !light
                } else {
                    light
                }
            }
            if (nextState == expected) {
                return nextDepth
            }
            if (visited.contains(nextState)) {
                return@forEach
            }
            visited[nextState] = nextDepth
            queue.add(nextState)
        }
    }
}

fun main() {
    val file = Files.readString(Path.of("input.txt"))
    val machines = file.lines()
        .map { line ->
            val parts = line.split(" ")
            val lights = parts.first().removePrefix("[").removeSuffix("]").map {
                when (it) {
                    '.' -> false
                    '#' -> true
                    else -> throw IllegalArgumentException("Unsupported light state $it")
                }
            }
            val buttons = parts.subList(1, parts.lastIndex).map { button ->
                button.removePrefix("(").removeSuffix(")").split(",").map { it.toInt() }
            }
            val joltageRequirements = parts.last().removePrefix("{").removeSuffix("}").split(",").map { it.toInt() }
            Machine(
                lights,
                buttons,
                joltageRequirements
            )
        }
    puzzle1(machines)
}
