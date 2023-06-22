export const to_full_date = (timestamp: number): String => {
    const date = new Date(timestamp);
    const month = date.toLocaleString('en-US', { month: 'long' });
    const day = date.getDate();
    const year = date.getFullYear();
  
    return `${month}, ${day}, ${year}`;
}