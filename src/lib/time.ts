import { format, formatDistance } from "date-fns";

export function formatDate(dateTime: number) {
  return format(dateTime * 1000, "HH:mm E, PP");
}

export function formatDateShort(dateTime: number) {
  return format(dateTime * 1000, "E, PP");
}

export function formatTimeAgo(dateTime: number) {
  return formatDistance(new Date(dateTime * 1000), new Date(), {
    addSuffix: true,
  });
}
