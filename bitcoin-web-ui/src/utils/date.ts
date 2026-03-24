/**
 * Safely format a date string or Date object to a localized string
 * @param date - Date string (ISO 8601) or Date object
 * @returns Formatted date string or "Invalid Date" if parsing fails
 */
export function formatDate(date: string | Date | null | undefined): string {
  if (!date) {
    return 'N/A';
  }

  try {
    let dateObj: Date;
    
    if (typeof date === 'string') {
      // Check if it's a malformed date (like "+57878-04-29T11:29:35Z" from backend bug)
      // This happens when milliseconds are treated as seconds
      if (date.startsWith('+') && date.includes('-')) {
        // Try to extract and fix: if year > 3000, it's likely milliseconds treated as seconds
        const yearMatch = date.match(/\+(\d+)-/);
        if (yearMatch) {
          // If year is unreasonably large (> 3000), this is likely a bug
          // For now, just show the raw timestamp or a fallback
          console.warn('Invalid date format detected (likely backend bug):', date);
          return 'Date format error';
        }
      }
      
      dateObj = new Date(date);
    } else {
      dateObj = date;
    }
    
    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      // Try to parse as Unix timestamp (milliseconds)
      if (typeof date === 'string') {
        const num = parseInt(date);
        if (!isNaN(num) && num > 0) {
          const timestampDate = new Date(num);
          if (!isNaN(timestampDate.getTime())) {
            return timestampDate.toLocaleString();
          }
        }
      }
      return 'Invalid Date';
    }

    // Check if date is unreasonably far in the future (likely a bug)
    const year = dateObj.getFullYear();
    if (year > 3000) {
      console.warn('Date is unreasonably far in the future (likely backend bug):', date, '->', dateObj);
      return 'Date format error';
    }

    return dateObj.toLocaleString();
  } catch (error) {
    console.error('Error formatting date:', error, date);
    return 'Invalid Date';
  }
}

/**
 * Format a date string to a more readable format
 * @param date - Date string (ISO 8601) or Date object
 * @returns Formatted date string with timezone
 */
export function formatDateWithTimezone(date: string | Date | null | undefined): string {
  if (!date) {
    return 'N/A';
  }

  try {
    const dateObj = typeof date === 'string' ? new Date(date) : date;
    
    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      return 'Invalid Date';
    }

    return dateObj.toLocaleString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      timeZoneName: 'short',
    });
  } catch (error) {
    console.error('Error formatting date:', error, date);
    return 'Invalid Date';
  }
}

