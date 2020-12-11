#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct RustPerson {
  const char *name;
  uint32_t age;
  float height;
};

struct RustPeople {
  uintptr_t len;
  RustPerson *data;
};

extern "C" {

uint32_t addition(uint32_t a, uint32_t b);

char *concatenate(char *s1, char *s2);

void free_people(RustPeople people);

void free_person(RustPerson person);

void free_string(char *string);

RustPeople get_people();

RustPerson get_person();

char *get_str();

void print_rust();

} // extern "C"
