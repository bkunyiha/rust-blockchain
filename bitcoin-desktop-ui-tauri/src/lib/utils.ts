/**
 * Format satoshis to BTC string
 */
export function formatBTC(satoshis: number): string {
  if (typeof satoshis !== 'number') return '0 BTC';
  const btc = satoshis / 100_000_000;
  return `${btc.toFixed(8)} BTC`;
}

/**
 * Truncate hash with ellipsis
 */
export function truncateHash(hash: string, chars: number = 12): string {
  if (hash.length <= chars) return hash;
  const half = Math.floor(chars / 2);
  return `${hash.slice(0, half)}...${hash.slice(-half)}`;
}

/**
 * Format ISO date string to locale date
 */
export function formatDate(iso: string): string {
  try {
    const date = new Date(iso);
    return date.toLocaleString();
  } catch {
    return iso;
  }
}

/**
 * Merge className strings (like clsx)
 */
export function cn(...classes: (string | undefined | null | false)[]): string {
  return classes.filter(Boolean).join(' ');
}

/**
 * Format large numbers with commas
 */
export function formatNumber(num: number): string {
  return num.toLocaleString();
}

/**
 * Safe JSON stringify with indentation
 */
export function stringifyJSON(obj: any, indent: number = 2): string {
  try {
    return JSON.stringify(obj, null, indent);
  } catch {
    return String(obj);
  }
}

/**
 * Debounce function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}
