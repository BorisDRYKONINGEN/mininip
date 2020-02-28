#include <stdint.h>

/**
 * \file mininip.h
 * \brief The header file to include to use Mininip
 * \note The documentation might be incomplete or outdated, see the Rust documentation
*/

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief An opaque type to handle a parser
 * \warning must be destroyed through mininipDestroyParser or mininipGetParserData
 * \see mininipNewParser
 * \see mininipDestroyParser
 * \see mininipGetParserData
*/
struct MininipParser;
typedef struct MininipParser MininipParser;

/**
 * \brief An opaque type to handle datas returned from a parser
 * \warning must be destroyed through mininipDestroyParserData
 * \see mininipGetParserData
 * \see mininipParseFile
 * \see mininipDestroyParserData
*/
struct MininipData;
typedef struct MininipData MininipData;

/**
 * \brief An enumeration for handling different types of error
*/
typedef enum MininipErrorKind {
    /// \brief No error has occured
    MININIP_NO_ERROR = 0,
    /// \brief An error has occured while parsing
    MININIP_PARSE_ERROR,
    /// \brief an error has occured while in an I/O stream
    MININIP_IO_ERROR,
    /// \brief another kind of error has occured
    MININIP_RUNTIME_ERROR,
} MininipErrorKind;

/**
 * \brief An error returned from the Rust module
 * \warning must be destroyed through mininipDestroyError unless `kind` is `MININIP_NO_ERROR` (which is accepted though)
 * \warning you must **not** allocate the memory of `msg` by yourself if creating an instance manually
 * \see mininipDestroyError
*/
typedef struct MininipError {
    /// \brief the error message (if any) or `NULL`
    const char* msg;
    /// \brief the kind of error
    MininipErrorKind kind;
} MininipError;

/**
 * \brief A structure which holds the raw content written into an INI file
 * \note it is not a simple pointer to make you aware you must destroy it through
 * Mininip since Mininip allocated it
*/
typedef struct MininipRawValue {
    const char* ptr;
} MininipRawValue;

/**
 * \brief A structure which holds a quoted string read from an INI file
 * \note it is not a simple pointer to make you aware you must destroy it through
 * Mininip since Mininip allocated it
*/
typedef struct MininipStrValue {
    const char* ptr;
} MininipStrValue;

/**
 * \brief A 64-bits sized integer read from an INI file
*/
typedef uint64_t MininipIntValue;

/**
 * \brief A 64-bits sized floating point number read from an INI file
*/
typedef double MininipFloatValue;

/**
 * \brief An int-sized boolean read from an INI file
 * \note as a boolean, the only two supported values are MININIP_TRUE and
 * MININIP_FALSE
*/
typedef int MininipBoolValue;

/**
 * brief The `true` variant of the boolean
*/
#define MININIP_TRUE 1
/**
 * \brief The `false` variant of the boolean
*/
#define MININIP_FALSE 0

/**
 * \brief An union which can hold a value of all the types supported by Mininip
 * \note You should use it through a MininipEntry
 * \see MininipEntry
*/
typedef union MininipValue {
    MininipRawValue raw;
    MininipStrValue string;
    MininipIntValue integer;
    MininipFloatValue floating;
    MininipBoolValue boolean;
} MininipValue;

/**
 * \brief An enumeration which can represent all the types supported by Mininip
 * \note You should use it through a MininipEntry
 * \see MininipEntry
*/
typedef enum MininipType {
    MININIP_TYPE_RAW,
    MININIP_TYPE_STR,
    MININIP_TYPE_INT,
    MININIP_TYPE_FLOAT,
    MININIP_TYPE_BOOL,
} MininipType;

/**
 * \brief An entry which is a value associated to a key and a section
 * \see mininipGetEntry
 * \see mininipDestroyEntry
*/
typedef struct MininipEntry {
    MininipValue value;
    MininipType valueType;
} MininipEntry;

/**
 * \brief A handle to a data tree. A tree is a more user-friendly data-type to represent the data returned by mininipGetParserData
 * \see mininipCreateTreeFromData to create it
 * \see mininipDestroyTree to destroy it
 * \see mininipGetDataFromTree to extract the MininipData owned
*/
typedef struct MininipTree MininipTree;

/**
 * \brief Creates a new handle to a parser
 * \returns a pointer to a new parser
 * \see MininipParser
*/
MininipParser* mininipNewParser(void);

/**
 * \brief Destroys a parser and invalidates its handle
 * \param parser the parser to destroy
*/
void mininipDestroyParser(MininipParser* parser);

/**
 * \brief Destroys a parser and returns the parsed data
 * \param parser the parser to destroy and to extract the data from
 * \returns the MininipData extracted from `parser`
*/
MininipData* mininipGetParserData(MininipParser* parser);

/**
 * \brief Destroys the datas returned by a parser
 * \param data the datas to destroy
*/
void mininipDestroyParserData(MininipData* data);

/**
 * \brief Destroys an error returned by any function of this library
 * \param err the error to destroy
 * \note despite the MininipError tells an error has not to be destroyed if `kind` is `MININIP_NO_ERROR`, you can pass it all the initialized instances of MininipError
*/
void mininipDestroyError(MininipError* err);

/**
 * \brief Parses a `path`-named file
 * \param path the name of the file
 * \param data a pointer to the handle to the data to get from the parser
 * \returns a MininipError to check whether an error occured
*/
MininipError mininipParseFile(const char* path, MininipData** data);

/**
 * \brief Creates an Entry found in an INI file
 * \param data the data set to search in
 * \param section the (optional) name of the section
 * \param key the name of the key
 * \param entry a pointer to the MininipEntry to assign
 * \returns `MININIP_TRUE` in case of success, `MININIP_FALSE` in case of error.
 * 
 * An error means either
 * - An invalid value for `section` or `key` (note the variable cannot be found because it cannot exist in this case)
 * - A variable not found
 * - An allocation or conversion error
*/
MininipBoolValue mininipGetEntry(MininipData* data, const char* section, const char* key, MininipEntry* entry);

/**
 * \brief Destroys a MininipEntry by freeing all the ressources
 * \param entry the entry to destroy
*/
void mininipDestroyEntry(MininipEntry* entry);

/**
 * \brief Creates a new MininipTree from an existing MininipData
 * \param data the data to build a MininipTree from. It will be invalidated
 * \returns a MininipTree holding and representig `data` or a null pointer if any error occurs (which is always any runtime error such as memory allocation failure)
*/
MininipTree* mininipCreateTreeFromData(MininipData* data);

/**
 * \brief Destroys a MininipTree and the MininipData held
 * \param tree the tree to destroy
*/
void mininipDestroyTree(MininipTree* tree);

/**
 * \brief Releases the MininipData used by a MininipTree
 * \param tree the MininipTree to destroy and to extract data from
 * \returns a pointer to that MininipData or `NULL` if a memory allocation failed
*/
MininipData* mininipGetDataFromTree(MininipTree* tree);

#ifdef __cplusplus
}
#endif
