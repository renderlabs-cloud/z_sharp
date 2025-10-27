#pragma once

/**
 * # Constants
 */

#define HASH #
#define O
#define TAB \t
#define N \n

/**
 * # Manipulation Macros
 */

#define EXPAND(...) __VA_ARGS__
#define NEXT(...) DEFER(__VA_ARGS__)
#define STRINGIFY(...) #__VA_ARGS__
#define STRINGIFYX(...) STRINGIFY(__VA_ARGS__)

#define COUNT(...)

#define CONCAT_IMPL(a, ...) a##__VA_ARGS__
#define CONCAT(...) CONCAT_IMPL(__VA_ARGS__)

#define EMPTY(...)
#define DEFER(...) __VA_ARGS__ EMPTY()
#define OBSTRUCT(...) __VA_ARGS__ DEFER(EMPTY)()
#define EXPAND(...) __VA_ARGS__

#define M(m, ...) EXPAND(HASH)m EXPAND(__VA_ARGS__)

/**
 * # Logic
 */

