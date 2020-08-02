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
  RustPerson person1 = { "A", 51, 1.67 };
  std::cout << "Person 1: " << '\n';
  std::cout << "  Name: " << person1.name << '\n';
  std::cout << "  Age: " << person1.age << '\n';
  std::cout << "  Height: " << person1.height << '\n';
  //
  std::cout << "Getting a person from Rust function:" << '\n';
  RustPerson person2 = get_person();
  std::cout << "Person 2: " << '\n';
  std::cout << "  Name: " << person2.name << '\n';
  std::cout << "  Age: " << person2.age << '\n';
  std::cout << "  Height: " << person2.height << '\n';
  //
  std::cout << "Getting a list from Rust function:" << '\n';
  auto people = get_people();
  int j = 3;
  for (size_t i = 0; i < people.size; i++) {
    std::cout << "Person " << j++ << ": \n";
    std::cout << "  Name: " << people.list[i].name << '\n';
    std::cout << "  Age: " << people.list[i].age << '\n';
    std::cout << "  Height: " << people.list[i].height << '\n';
  }
  return EXIT_SUCCESS;
}
