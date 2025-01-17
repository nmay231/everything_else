// Source: https://devhints.io/jsdoc
// I want an offline copy, and to filter it down to the most useful things, okay...

/**
 * Function description
 *
 * @param {string} input1 - Parameter description (optional)
 * @param {string} [a] - Optional value
 * @param {string=} b - Also, an optional value
 * @param {string} [c="my default value"] - Optional with a default
 * @param {...string} etc - Variadic parameters
 * @return {string} A good string
 *
 * @example
 *
 *     foo('hello')
 */

function foo(input1, a, b, c="my default value", ...etc) {
  return input1
}

/** @type {number | null} - Type a variable */
const asdf1 = null;

/** @const {number | null} - Or make it explicitly constant, I guess */
const asdf2 = null;

/**
 * Typedef an object type explicitly: `type Song = { ... };`
 * @typedef {Object} Song
 * @property {string} title - The title
 * @property {string} artist - The artist
 * @property {number} year - The year
 */

/**
 * Or use shorthand
 * @typedef {{title: string, artist: string, year: number}} Song
 */

/**
 * @type {null} - Look mommy! I can link to types like {@link Song} in documentation!
 */
let todo = null;

// Type cast (It acts sorta like a function, hence the parenthesis)
let mySong = /** @type {Song} */ (null);


// Import a type with import() (typescript-only)
/** @typedef {import('./Foo').default} Bar */

// or this (I'm not getting syntax highlighting, though, so I don't know if it
// actually works)
/** @import { Bar } from "./Foo.js" */
