/**
 * Query params injection
 * @param key
 * @returns
 */
export function Q(key: string) {
  return `query(${key})`;
}

/**
 * Path params injection
 * @param key
 * @returns
 */
export function P(key: string) {
  return `path(${key})`;
}

/**
 * Body params injection
 * @param key
 * @returns
 */
export function B(key: string) {
  return `body(${key})`;
}
