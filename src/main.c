#include <stdio.h>

#include "mininip.h"

int main(void) {
    MininipParser* parser = mininipNewParser();
    printf("%p\n", parser);

    MininipData* data = mininipGetParserData(parser);
    printf("%p\n", data);

    mininipDestroyParserData(data);
    return 0;
}
