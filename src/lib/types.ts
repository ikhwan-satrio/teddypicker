export interface DirEntryInfo {
  name: string;
  path: string;
  is_dir: boolean;
  extension: string | null;
}

export interface ExtensionInfo {
  id: string;
  name: string;
  version: string;
  type: 'theme' | 'plugin';
  enabled: boolean;
}

export interface TrashEntry {
  id: string;
  name: string;
  original_path: string;
  time_deleted: number;
  is_dir: boolean;
}
