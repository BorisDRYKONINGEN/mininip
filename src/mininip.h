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

#ifdef __cplusplus
}
#endif
