#ifdef __cplusplus
extern "C" {
#endif

struct MininipParser;
typedef struct MininipParser MininipParser;

struct MininipData;
typedef struct MininipData MininipData;

typedef enum MininipErrorKind {
    MININIP_NO_ERROR = 0,
    MININIP_PARSE_ERROR,
    MININIP_IO_ERROR,
    MININIP_RUNTIME_ERROR,
} MininipErrorKind;

typedef struct MininipError {
    const char* msg;
    MininipErrorKind kind;
} MininipError;

MininipParser* mininipNewParser(void);
void mininipDestroyParser(MininipParser*);
MininipData* mininipGetParserData(MininipParser*);
void mininipDestroyParserData(MininipData*);
MininipError mininipParseFile(const char* path, MininipData**);

#ifdef __cplusplus
}
#endif
