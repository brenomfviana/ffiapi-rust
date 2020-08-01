#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

extern "C" {

uint32_t addition(uint32_t a, uint32_t b);

const char *get_string();

void rust();

} // extern "C"
