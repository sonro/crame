#if defined HEADERS
#include <string.h>
#elif defined TESTS

TEST("strlen returns length of string") {
    ASSERT(strlen("") == 0);
    ASSERT(strlen("foo") == 3);
}

#endif