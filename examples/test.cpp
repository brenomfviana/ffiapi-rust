#include <iostream>
#include "ffiapi.h"

int main() {
  rust();
  std::cout << addition(1, 1) << '\n';
  std::cout << addition(2, 2) << '\n';
  return EXIT_SUCCESS;
}
