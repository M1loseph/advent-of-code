#include <iostream>
#include <print>

#include "graph.hpp"
#include "graph_loader.hpp"

void puzzle_1(Graph& graph) {
  uint64_t jumps = graph.keep_jumping_till_target("AAA", std::regex("^ZZZ$"));
  std::println("Puzzle 1: jumps = {}", jumps);
}

void puzzle_2_naive(Graph& graph) {
    uint64_t jumps = graph.keep_jumping_till_all_on_target_naive_solution(
        std::regex(".*A$"), std::regex(".*Z$"));

    std::println("Puzzle 2: naive solution jumps = {}", jumps);
}

void puzzle_2(Graph& graph) {
  uint64_t jumps = graph.keep_jumping_till_all_on_target_least_common_multiple(
      std::regex(".*A$"), std::regex(".*Z$"));

  std::println("Puzzle 2: least common multiple solution jumps = {}", jumps);
}

int main() {
  GraphLoader graphLoader;
  auto graph = graphLoader.loadGraph();
  if (!graph) {
    std::println("Graph loading failed: {}", graph.error());
    return 1;
  }
  graph->print_graph();
  puzzle_1(*graph);
  puzzle_2(*graph);
  // This solution literally takes forever to run
  puzzle_2_naive(*graph);
}
