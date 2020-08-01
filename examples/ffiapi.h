#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct RustPerson {
  const char *name;
  uint32_t age;
  float height;
};

extern "C" {

uint32_t addition(uint32_t a, uint32_t b);

RustPerson get_person();

const char *get_string();

void rust();

} // extern "C"
