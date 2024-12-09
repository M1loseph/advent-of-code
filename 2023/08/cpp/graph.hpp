#ifndef GRAPH_HPP
#define GRAPH_HPP

#include <cassert>
#include <expected>
#include <memory>
#include <regex>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "error.hpp"

enum class Direction { Left, Right };

class Node {
 public:
  Node(std::string name) : name(std::move(name)), edges(nullptr, nullptr){};
  Node(std::string &&name) : name(name), edges(nullptr, nullptr){};

  const std::string name;
  std::pair<Node *, Node *> edges;
};

class Graph {
 public:
  explicit Graph() = default;

  void add_direction(Direction direction);
  void add_node(std::string name);
  std::expected<void, Error> add_edge(std::string_view from,
                                      std::string_view left,
                                      std::string_view right);

  void print_graph();

  uint64_t keep_jumping_till_target(std::string_view from, std::regex to);
  // This naive solution takes forever to run
  uint64_t keep_jumping_till_all_on_target_naive_solution(std::regex from, std::regex to);
  uint64_t keep_jumping_till_all_on_target_least_common_multiple(std::regex from, std::regex to);

 private:
  Node *getNode(std::string_view name); 

  std::vector<std::unique_ptr<Node>> _nodes;
  std::vector<Direction> _directions;
};

#endif
