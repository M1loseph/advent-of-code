#include "graph.hpp"

#include <algorithm>
#include <chrono>
#include <print>

void Graph::add_direction(Direction direction) {
  _directions.push_back(direction);
}

void Graph::add_node(std::string name) {
  _nodes.push_back(std::make_unique<Node>(name));
}

std::expected<void, Error> Graph::add_edge(std::string_view from,
                                           std::string_view left,
                                           std::string_view right) {
  Node *from_ptr = getNode(from);
  Node *left_ptr = getNode(left);
  Node *right_ptr = getNode(right);
  if (!from_ptr || !left_ptr || !right_ptr) {
    return std::unexpected(Error("At last one node not found"));
  }
  from_ptr->edges = std::make_pair(left_ptr, right_ptr);
  return std::expected<void, Error>();
}

void Graph::print_graph() {
  for (const auto &direction : _directions) {
    auto directionLetter = direction == Direction::Left ? 'L' : 'R';
    std::print("{}", directionLetter);
  }
  std::println("");
  for (const auto &node : _nodes) {
    auto left = node->edges.first ? node->edges.first->name : "null";
    auto right = node->edges.second ? node->edges.second->name : "null";
    std::println("{}: ({}, {})", node->name, left, right);
  }
}

uint64_t Graph::keep_jumping_till_target(std::string_view from, std::regex to) {
  uint64_t jumps = 0;

  Node *current_node = getNode(from);

  do {
    auto current_direction = _directions[jumps % _directions.size()];
    if (current_direction == Direction::Left) {
      current_node = current_node->edges.first;
    } else {
      current_node = current_node->edges.second;
    }
    jumps += 1;
  } while (!std::regex_match(current_node->name, to));

  return jumps;
}

Node *Graph::getNode(std::string_view name) {
  for (const auto &node : _nodes) {
    if (node->name == name) {
      return node.get();
    }
  }
  assert(false && "Node not found");
  return nullptr;
}

bool all_match(const std::vector<Node *> nodes, std::regex &to) {
  for (const auto &node : nodes) {
    if (!std::regex_match(node->name, to)) {
      return false;
    }
  }
  return true;
}

uint64_t Graph::keep_jumping_till_all_on_target_naive_solution(std::regex from,
                                                               std::regex to) {
  std::vector<Node *> nodes;

  for (const auto &node : _nodes) {
    if (std::regex_match(node->name, from)) {
      nodes.push_back(node.get());
    }
  }

  const uint64_t expected_iterations = 14935034899483;
  auto now = std::chrono::system_clock::now();
  uint64_t jumps = 0;

  do {
    for (auto &current_node : nodes) {
      auto current_direction = _directions[jumps % _directions.size()];
      if (current_direction == Direction::Left) {
        current_node = current_node->edges.first;
      } else {
        current_node = current_node->edges.second;
      }
    }
    jumps += 1;
    if (now + std::chrono::seconds(1) < std::chrono::system_clock::now()) {
      now = std::chrono::system_clock::now();
      double percent = static_cast<double>(jumps) * 100 /
                       static_cast<double>(expected_iterations);
      std::println("Jumps: {} ({:.2f}%)", jumps, percent);
    }
  } while (!std::all_of(nodes.begin(), nodes.end(), [&to](const auto &node) {
    return std::regex_match(node->name, to);
  }));

  return jumps;
}

uint64_t Graph::keep_jumping_till_all_on_target_least_common_multiple(
    std::regex from, std::regex to) {
  std::vector<uint64_t> nodes_jumps;

  for (const auto &node : _nodes) {
    if (std::regex_match(node->name, from)) {
      nodes_jumps.push_back(keep_jumping_till_target(node->name, to));
    }
  }

  auto lcm = [](uint64_t a, uint64_t b) {
    uint64_t gcd = 1;
    for (uint64_t i = 1; i <= a && i <= b; ++i) {
      if (a % i == 0 && b % i == 0) {
        gcd = i;
      }
    }
    return a * b / gcd;
  };

  for (int i = 0; i < nodes_jumps.size() - 1; i++) {
    const auto lcm_two_numbers = lcm(nodes_jumps[i], nodes_jumps[i + 1]);
    nodes_jumps[i + 1] = lcm_two_numbers;
  }

  return nodes_jumps.back();
}
