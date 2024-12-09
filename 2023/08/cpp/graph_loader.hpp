#ifndef GRAPH_LOADER_HPP
#define GRAPH_LOADER_HPP

#include <expected>

#include "error.hpp"
#include "graph.hpp"

class GraphLoader;

class Parser {
 public:
  virtual std::expected<void, Error> parseLine(Graph *graph,
                                               GraphLoader *graph_loader,
                                               const std::string &line) = 0;
};

class DirectionsParser : public Parser {
 public:
  std::expected<void, Error> parseLine(Graph *graph, GraphLoader *graph_loader,
                                       const std::string &line) override;
};

class NodeParser : public Parser {
 public:
  std::expected<void, Error> parseLine(Graph *graph, GraphLoader *graph_loader,
                                       const std::string &line) override;
};

class EmptyLineParser : public Parser {
 public:
  std::expected<void, Error> parseLine(Graph *graph, GraphLoader *graph_loader,
                                       const std::string &line) override;
};

class Edge {
 public:
  Edge(std::string &&from, std::string &&left, std::string &&right)
      : from(from), left(left), right(right){};

  std::string from;
  std::string left;
  std::string right;
};

class GraphLoader {
 public:
  std::expected<Graph, Error> loadGraph();

 private:
  friend class NodeParser;

  Parser *chooseParser(std::string_view line);

  std::vector<Edge> _cached_edges;

  bool _directions_loaded = false;
  DirectionsParser _directions_parser;
  NodeParser _node_parser;
  EmptyLineParser _empty_line_parser;
};

#endif
