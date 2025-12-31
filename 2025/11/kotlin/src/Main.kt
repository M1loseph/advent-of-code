import java.nio.file.Files
import java.nio.file.Path


data class Graph(
    val neighbours: Map<String, List<String>>
)

fun puzzle1(graph: Graph) {
    val start = "you"
    val end = "out"

    fun dfs(node: String, cache: MutableMap<String, Long>): Long {
        if (node == end) {
            return 1
        }
        val cachedValue = cache[node]
        if (cachedValue != null) {
            return cachedValue
        }
        val neighbours = graph.neighbours[node]!!
        return neighbours.sumOf { dfs(it, cache) }.also {
            cache[node] = it
        }
    }

    val numberOfPaths = dfs(start, HashMap())
    println("Puzzle 1 number of paths: $numberOfPaths")
}

data class CacheKey(
    val node: String,
    val visitedDAC: Boolean,
    val visitedFFT: Boolean
)

fun puzzle2(graph: Graph) {
    val start = "svr"
    val end = "out"

    fun dfs(node: String, visitedDAC: Boolean, visitedFFT: Boolean, cache: MutableMap<CacheKey, Long>): Long {
        if (node == end) {
            return if (visitedDAC && visitedFFT) 1 else 0
        }
        val cacheKey = CacheKey(node, visitedDAC, visitedFFT)
        val cachedValue = cache[cacheKey]
        if (cachedValue != null) {
            return cachedValue
        }
        val neighbours = graph.neighbours[node]!!
        val currentDAC = node == "dac"
        val currentFFT = node == "fft"
        return neighbours.sumOf {
            dfs(it, visitedDAC || currentDAC, visitedFFT || currentFFT, cache)
        }.also {
            cache[cacheKey] = it
        }
    }

    val numberOfPaths = dfs(start, visitedDAC = false, visitedFFT = false, cache = HashMap())
    println("Puzzle 2 number of paths: $numberOfPaths")
}

fun main() {
    val file = Files.readString(Path.of("input.txt"))
    val neighbours = file.lines().associate {
        val elements = it.split(":")
        val node = elements[0].trim()
        val neighbours = elements[1].trim().split(" ")
        node to neighbours
    }
    val graph = Graph(neighbours)
    puzzle1(graph)
    puzzle2(graph)
}
