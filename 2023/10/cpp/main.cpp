#include <cstdint>
#include <print>
#include <vector>
#include <fstream>
#include <array>

using Reading = int64_t;
using History = std::vector<Reading>;

template<typename T>
void printVec(const std::vector<T>& vec) {
  for(auto& e : vec) {
    std::print("{} ", e);
  }
  std::println();
}

int main() {
  std::ifstream file;
  file.open("input.txt");
  
  std::vector<History> histories;
  for(std::array<char, 1024> buffer{}; file.getline(&buffer[0], buffer.size());) {
    History history;
    int nextValue{};
    for(auto element : buffer) {
      if(element == '\0') {
        break;
      }
      if(element == ' ') {
        history.push_back(nextValue);
        nextValue = 0;
        continue;
      }
      int charAsInt;
      switch(element) {
        case '0': 
          charAsInt = 0;
          break;
        case '1': 
          charAsInt = 1;
          break;
        case '2': 
          charAsInt = 2;
          break;
        case '3': 
          charAsInt = 3;
          break;
        case '4': 
          charAsInt = 4;
          break;
        case '5': 
          charAsInt = 5;
          break;
        case '6': 
          charAsInt = 6;
          break;
        case '7': 
          charAsInt = 7;
          break;
        case '8': 
          charAsInt = 8;
          break;
        case '9': 
          charAsInt = 9;
          break;
        default:
          std::println("Got unexpected character {}", (uint8_t) element);
          std::exit(1);
      }
      nextValue = nextValue * 10 + charAsInt;
    }
    histories.push_back(std::move(history));
  }
  for(auto& history : histories) {
    printVec(history);
  }
}

