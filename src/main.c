#include <stdio.h>

#include "mininip.h"

int main(int argc, const char* const* argv) {
    MininipParser* parser = mininipNewParser();
    printf("%p\n", (void*) parser);

    MininipData* data = mininipGetParserData(parser);
    printf("%p\n", (void*) data);

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

        mininipDestroyError(&err);

        return -1;
    }

    printf("%p\n", (void*) fileDatas);

    MininipEntry entry;
    if (mininipGetEntry(fileDatas, NULL, "author", &entry) == MININIP_FALSE) {
        fputs("`author` key not found in the file\n", stderr);
        goto destroyData;
    }

    switch (entry.valueType) {
    case MININIP_TYPE_RAW:
        fputs("Warning, the type `Raw` has been used for `author`\n", stderr);
        printf("The author is %s !\n", entry.value.raw.ptr);
        break;

    case MININIP_TYPE_STR:
        printf("The author is %s !\n", entry.value.string.ptr);
        break;

    default:
        fputs("Invalid type for `author` !\n", stderr);
        goto destroyEntry;
        break;
    }

    mininipDestroyEntry(&entry);
    mininipDestroyParserData(fileDatas);

    return 0;

destroyEntry:
    mininipDestroyEntry(&entry);

destroyData:
    mininipDestroyParserData(fileDatas);

    return -1;
}
