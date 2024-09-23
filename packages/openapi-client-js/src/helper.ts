/**
 * Query params injection
 * @param key
 * @returns
 */
export function Q(key: string) {
  return `q/${key}`;
}

/**
 * Path params injection
 * @param key
 * @returns
 */
export function P(key: string) {
  return `p/${key}`;
}

/**
 * Body params injection
 * @param key
 * @returns
 */
export function B(key: string) {
  return `b/${key}`;
}
