import type { DirEntryInfo } from '$lib/types';
import {
  Folder,
  File,
  FileText,
  FileCode,
  FileImage,
  FileVideo,
  FileAudio,
  FileArchive,
  FileJson,
} from 'lucide-svelte';

type IconComponent = typeof Folder;

const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'bmp', 'ico'];
const videoExts = ['mp4', 'avi', 'mov', 'mkv', 'webm'];
const audioExts = ['mp3', 'wav', 'ogg', 'flac', 'aac'];
const archiveExts = ['zip', 'tar', 'gz', '7z', 'rar', 'xz'];
const codeExts = ['js', 'ts', 'tsx', 'jsx', 'py', 'rs', 'go', 'java', 'c', 'cpp', 'h'];
const textExts = ['md', 'txt', 'log', 'csv'];

export function getFileIcon(entry: DirEntryInfo): IconComponent {
  if (entry.is_dir) return Folder;
  const ext = entry.extension?.toLowerCase() || '';
  if (imageExts.includes(ext)) return FileImage;
  if (videoExts.includes(ext)) return FileVideo;
  if (audioExts.includes(ext)) return FileAudio;
  if (archiveExts.includes(ext)) return FileArchive;
  if (codeExts.includes(ext)) return FileCode;
  if (['json'].includes(ext)) return FileJson;
  if (textExts.includes(ext)) return FileText;
  return File;
}

export function getFileColor(entry: DirEntryInfo): string {
  if (entry.is_dir) return 'text-blue-500';
  const ext = entry.extension?.toLowerCase() || '';
  if (imageExts.includes(ext)) return 'text-purple-500';
  if (videoExts.includes(ext)) return 'text-red-500';
  if (audioExts.includes(ext)) return 'text-green-500';
  if (['js', 'ts', 'tsx'].includes(ext)) return 'text-yellow-500';
  if (['py'].includes(ext)) return 'text-blue-400';
  if (['rs'].includes(ext)) return 'text-orange-500';
  if (['json'].includes(ext)) return 'text-emerald-500';
  if (['md'].includes(ext)) return 'text-sky-500';
  return 'text-muted-foreground';
}
