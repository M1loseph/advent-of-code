#include "graph_loader.hpp"

#include <fstream>
#include <iostream>

std::expected<Graph, Error> GraphLoader::loadGraph() {
  std::ifstream file("input/input.txt");
  if (!file) {
    return std::unexpected(Error("Could not open the file"));
  }

  Graph graph;

  for (std::string line; std::getline(file, line);) {
    auto line_parser = chooseParser(line);
    auto result = line_parser->parseLine(&graph, this, line);
    if (!result) {
      return std::unexpected(result.error());
    }
  }

  for (const auto &edge : _cached_edges) {
    graph.add_edge(edge.from, edge.left, edge.right);
  }

  return graph;
}

Parser *GraphLoader::chooseParser(std::string_view line) {
  if (line.empty()) {
    return &_empty_line_parser;
  } else if (!_directions_loaded) {
    _directions_loaded = true;
    return &_directions_parser;
  } else {
    return &_node_parser;
  }
}

std::expected<void, Error> DirectionsParser::parseLine(
    Graph *graph, GraphLoader *graph_loader, const std::string &line) {
  for (const auto direction : line) {
    switch (direction) {
      case 'L':
        graph->add_direction(Direction::Left);
        break;
      case 'R':
        graph->add_direction(Direction::Right);
        break;
      default:
        return std::unexpected(Error("Invalid direction: " + direction));
    }
  }
  return std::expected<void, Error>();
}

std::expected<void, Error> EmptyLineParser::parseLine(Graph *graph,
                                                      GraphLoader *graph_loader,
                                                      const std::string &line) {
  return std::expected<void, Error>();
}

std::expected<void, Error> NodeParser::parseLine(Graph *graph,
                                                 GraphLoader *graph_loader,
                                                 const std::string &line) {
  static std::regex line_regex("[A-Z]{3}\\s=\\s\\([A-Z]{3},\\s[A-Z]{3}\\)");

  if (!std::regex_match(line, line_regex)) {
    return std::unexpected(Error("Invalid line: " + line));
  }

  std::string nodeName = line.substr(0, 3);
  std::string leftName = line.substr(7, 3);
  std::string rightName = line.substr(12, 3);

  graph_loader->_cached_edges.emplace_back(
      std::move(nodeName), std::move(leftName), std::move(rightName));

  graph->add_node(nodeName);
  return {};
}
