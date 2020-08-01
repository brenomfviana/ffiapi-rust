#include <iostream>
#include <string>
#include "ffiapi.h"

int main() {
  rust();
  std::cout << addition(1, 1) << '\n';
  std::cout << addition(2, 2) << '\n';
  std::string str(get_string());
  std::cout << "String from Rust: " << str << '\n';
  return EXIT_SUCCESS;
}
