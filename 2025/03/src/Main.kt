import java.nio.file.Files
import java.nio.file.Path

fun puzzle1FindLargetsOutputJoltage(bank: List<Int>): Long {
    var largest = 0
    for (i in 0 until bank.size - 1) {
        val decimal = bank[i] * 10
        val largestDecimal = largest - (largest % 10)
        if (decimal < largestDecimal) {
            continue
        }
        for (j in i + 1 until bank.size) {
            val result = decimal + bank[j]
            if (result > largest) {
                largest = result
            }
        }
    }
    return largest.toLong()
}

fun puzzle1(banks: List<List<Int>>) {
    val result = banks.sumOf { puzzle1FindLargetsOutputJoltage(it) }
    println("Puzzle 1 result: $result")
}

fun power(baseVal: Int, exponentVal: Int): Long {
    return if (exponentVal != 0) {
        baseVal * power(baseVal, exponentVal - 1)
    } else {
        1
    }
}

fun puzzle2FindLargetsOutputJoltage(bank: List<Int>): Long {
    fun recurr(joltageSoFar: Long, remainingBank: List<Int>, remainingSize: Int): Long {
        val remaining = remainingBank.dropLast(remainingSize - 1)
        val max = remaining.max()
        return remaining
            .withIndex()
            .filter { element -> element.value == max }
            .maxOf { pair ->
                val index = pair.index
                val value = pair.value
                val remainingSize = remainingSize - 1
                val joltageSoFar = joltageSoFar + power(10, remainingSize) * value
                if (remainingSize == 0) {
                    joltageSoFar
                } else {
                    recurr(joltageSoFar, remainingBank.drop(index + 1), remainingSize)
                }
            }
    }
    return recurr(0, bank, 12)
}

fun puzzle2(banks: List<List<Int>>) {
    val result = banks.sumOf { it ->
        puzzle2FindLargetsOutputJoltage(it)
    }
    println("Puzzle 1 result: $result")
}

fun main() {
    val input = Files.readString(Path.of("input.txt"))
    val banks = input.lines().map {
        it.toCharArray().map { digit -> digit.digitToInt() }
    }
    puzzle1(banks)
    puzzle2(banks)
}