import type { ClassValue } from 'clsx'
import { clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export const APP_VERSION = '0.1-alpha.3'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
