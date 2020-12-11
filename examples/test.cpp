#include <iostream>
#include <memory.h>
#include "../include/ffiapi.h"

int main() {
  // Print in Rust
  std::cout << "Calling function from Rust:" << '\n';
  print_rust();

  // Sum in Rust
  std::cout << "Sum two numbers from Rust:" << '\n';
  std::cout << "  1 + 1 = " << addition(1, 1) << '\n';
  std::cout << "  2 + 2 = " << addition(2, 2) << '\n';

  // Get string from Rust
  std::cout << "Getting a string from Rust:" << '\n';
  char *rust_str_1 = get_str();
  std::cout << "  String from Rust: '" << rust_str_1 << "'\n";
  // Concatenate strings in Rust (this not allowed directly in C++)
  std::cout << "Change a Rust string in Rust side:" << '\n';
  char cpp_str1[] = " o/"; // This string will be cloned in Rust side
  std::cout << "  String from C++: '" << cpp_str1 << "'\n";
  char *rust_str_2 = concatenate(rust_str_1, cpp_str1);
  std::cout << "  Concatenated strings: '" << rust_str_2 << "'\n";
  // Free Rust pointer (all these operation must be in Rust library)
  free_string(rust_str_1);
  free_string(rust_str_2);

  // Create an object using a Rust struct
  std::cout << "Creating a person from Rust struct:" << '\n';
  RustPerson person1 = { "A", 51, 1.67 };
  std::cout << "  Person 1: " << '\n';
  std::cout << "    Name: " << person1.name << '\n';
  std::cout << "    Age: " << person1.age << '\n';
  std::cout << "    Height: " << person1.height << '\n';

  // Get an object from Rust
  std::cout << "Getting a person from Rust function:" << '\n';
  RustPerson person2 = get_person();
  std::cout << "  Person 2: " << '\n';
  std::cout << "    Name: " << person2.name << '\n';
  std::cout << "    Age: " << person2.age << '\n';
  std::cout << "    Height: " << person2.height << '\n';
  // Free Rust pointer (all these operation must be in Rust library)
  free_person(person2);

  // Get a list of objects from Rust
  std::cout << "Getting a list from Rust function:" << '\n';
  RustPeople people = get_people();
  int j = 3;
  for (size_t i = 0; i < people.len; i++) {
    std::cout << "  Person " << j++ << ": \n";
    std::cout << "    Name: " << people.data[i].name << '\n';
    std::cout << "    Age: " << people.data[i].age << '\n';
    std::cout << "    Height: " << people.data[i].height << '\n';
  }
  // Free Rust pointer (all these operation must be in Rust library)
  free_people(people);
  return EXIT_SUCCESS;
}
