#include <stdio.h>

#include "mininip.h"

int main(int argc, const char* const* argv) {
    MininipParser* parser = mininipNewParser();
    printf("%p\n", parser);

    MininipData* data = mininipGetParserData(parser);
    printf("%p\n", data);

    mininipDestroyParserData(data);

    if (argc <= 1)
        return 0;

    // From now, we know argc > 1 and so, argv[1] is valid
    MininipData* fileDatas = NULL;
    MininipError err = mininipParseFile(argv[1], &fileDatas);
    if (err.kind != MININIP_NO_ERROR) {
        fprintf(stderr, "An error has occured : 0x%04x ", err.kind);
        if (err.msg)
            fputs(err.msg, stderr);
        else
            fputs("NULL", stderr);
        fputc('\n', stderr);

        return -1;
    }

    printf("%p\n", fileDatas);
    mininipDestroyParserData(fileDatas);

    return 0;
}
