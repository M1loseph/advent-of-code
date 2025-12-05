import java.nio.file.Files
import java.nio.file.Path
import kotlin.math.max

data class Range(
    val start: Long,
    val end: Long,
)

fun puzzle1(ranges: List<Range>) {
    var invalidIdsSum: Long = 0
    for (range in ranges) {
        for (idNumeric in range.start..range.end) {
            val id = idNumeric.toString()
            if (id.length % 2 != 0) {
                continue
            }
            val chunks = id.chunked(id.length / 2)
            if (chunks[0] == chunks[1]) {
                invalidIdsSum += idNumeric
            }
        }
    }
    println("[Puzzle1] Sum of invalid ids: $invalidIdsSum")
}

fun puzzle2(ranges: List<Range>) {
    var invalidIdsSum: Long = 0
    for (range in ranges) {
        for (idNumeric in range.start..range.end) {
            val id = idNumeric.toString()
            for (i in 1..id.length / 2) {
                val chunked = id.chunked(i)
                val first = chunked[0]
                if (chunked.all { it == first }) {
                    invalidIdsSum += idNumeric
                    break
                }
            }
        }
    }
    println("[Puzzle2] Sum of invalid ids: $invalidIdsSum")
}

fun main() {
    val fileContent = Files.readString(Path.of("input.txt"))
    val ranges = fileContent.split(",").map {
        val rangeValues = it.split("-")
        Range(
            rangeValues[0].toLong(),
            rangeValues[1].toLong()
        )
    }
    puzzle1(ranges)
    puzzle2(ranges)
}