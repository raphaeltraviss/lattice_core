#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>



extern "C" {

void lc_free(char *s);

char *lc_init();

} // extern "C"
