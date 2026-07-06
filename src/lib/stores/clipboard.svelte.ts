export type ClipOp = 'copy' | 'cut';

let items = $state<string[]>([]);
let operation = $state<ClipOp>('copy');

export const fileClipboard = {
  get items() { return items; },
  get operation() { return operation; },
  get hasItems() { return items.length > 0; },

  copy(paths: string[]) {
    items = [...paths];
    operation = 'copy';
  },

  cut(paths: string[]) {
    items = [...paths];
    operation = 'cut';
  },

  clear() {
    items = [];
    operation = 'copy';
  },
};
