#include <iostream>
#include <string>
#include "ffiapi.h"

int main() {
  std::cout << "Calling function from Rust:" << '\n';
  rust();
  //
  std::cout << "Summing two numbers from Rust:" << '\n';
  std::cout << addition(1, 1) << '\n';
  std::cout << addition(2, 2) << '\n';
  //
  std::cout << "Getting a string from Rust:" << '\n';
  std::string str(get_string());
  std::cout << "String from Rust: '" << str << "'\n";
  str += " o/";
  std::cout << "Changed Rust string: '" << str << "'\n";
  //
  std::cout << "Creating a person from Rust struct:" << '\n';
  RustPerson person = { "Aurélio", 51, 1.67 };
  std::cout << "Person 1: " << '\n';
  std::cout << "  Name: " << person.name << '\n';
  std::cout << "  Age: " << person.age << '\n';
  std::cout << "  Height: " << person.height << '\n';
  //
  std::cout << "Getting a person from Rust function:" << '\n';
  person = get_person();
  std::cout << "Person 2: " << '\n';
  std::cout << "  Name: " << person.name << '\n';
  std::cout << "  Age: " << person.age << '\n';
  std::cout << "  Height: " << person.height << '\n';
  //
  return EXIT_SUCCESS;
}
