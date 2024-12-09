#ifndef ERROR_HPP
#define ERROR_HPP

#include <string>
#include <format>

class Error {
 public:
  explicit Error(std::string message) : _message(std::move(message)){};

  const std::string& message() const { return _message; }

 private:
  const std::string _message;
};

template <>
struct std::formatter<Error> {
  constexpr auto parse(std::format_parse_context& ctx) { return ctx.begin(); }

  auto format(const Error& error, std::format_context& ctx) const {
    return std::format_to(ctx.out(), "{}", error.message());
  }
};

#endif