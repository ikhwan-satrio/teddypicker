class DragStore {
  isDragging = $state(false);
  dragData = $state<{ name: string; path: string } | null>(null);

  startDrag(name: string, path: string) {
    this.isDragging = true;
    this.dragData = { name, path };
  }

  endDrag() {
    this.isDragging = false;
    this.dragData = null;
  }
}

export const dragStore = new DragStore();
