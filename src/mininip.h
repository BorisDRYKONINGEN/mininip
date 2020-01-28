#ifdef __cplusplus
extern "C" {
#endif

struct MininipParser;
typedef struct MininipParser MininipParser;

struct MininipData;
typedef struct MininipData MininipData;

MininipParser* mininipNewParser(void);
void mininipDestroyParser(MininipParser*);
MininipData* mininipGetParserData(MininipParser*);
void mininipDestroyParserData(MininipData*);

#ifdef __cplusplus
}
#endif
