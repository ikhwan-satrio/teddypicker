# Changelog

## [1.5.0] - 2026-07-11

### Added
- **Splash screen** — dark background loading screen with folder icon and spinner during startup
- **Dark mode anti-FOUC** — inline script applies `.dark` class from localStorage before first paint
- **ConfirmDialog `requireInput`** — type "yes" to confirm destructive actions (Empty Trash, Delete Permanently)
- **Trash toolbar** — Restore All and Empty Trash buttons with confirmation dialogs in trash view
- **RPM package build** — added RPM to CI/CD pipeline

### Fixed
- **White flash on startup** — native window `backgroundColor` set to `#0a0a0a`, `html/body` background dark
- **Duplicate trash buttons** — removed redundant Restore All / Clear All from FileGridToolbar (TrashView handles it)
- **Text overflow** — `original_path` in grid cards now truncates with ellipsis instead of overflowing

### Changed
- **ConfirmDialog** — added `requireInput` and `inputLabel` props, disabled confirm button until input matches
- **TrashView** — grid cards use `overflow-hidden` for proper text clipping

## [1.4.0] - 2026-07-08

### Added
- **Compress name dialog** — custom archive name input before compressing
- **Trash toolbar** — Restore All and Empty Trash buttons with confirmation dialogs
- **Reusable ConfirmDialog component** — custom confirmation modal for destructive actions
- **CI/CD pipeline** — GitHub Actions workflow for automated build & release (Linux + Windows)
- **Delete hotkey** — press `Delete` to trash selected files, with size-based permanent delete confirmation (>= 1 GB)

### Fixed
- **Compress "No such file" error** — auto-create destination directory before archiving
- **Compress "undefined" error** — improved error message extraction across all mutations
- **Error handling** — all mutations use `getErrorMessage()` helper for safe error display

### Changed
- **Archive operations** — compress/extract run on Tokio thread pool (`spawn_blocking`), UI stays responsive with real-time progress toasts
- **Component split** — FileGrid refactored into FileGridToolbar, FileGridView, FileListView, TrashView

## [1.3.0] - 2026-07-08

### Fixed
- **Production GTK crash** — replaced `wrapGAppsHook4` with `makeWrapper` for reliable GTK library wrapping in Nix package builds
- **Dev shell LD_LIBRARY_PATH** — GTK runtime libraries now available in development shell via `shellHook`

### Removed
- Removed unused `wrapGAppsHook4` from dev shell

## [1.2.0] - 2026-07-08

### Added
- **Archive support** — compress and extract ZIP, Tar, Tar.Gz, Tar.Bz2, Tar.XZ, Tar.Zst archives
- **Context menu actions** — Extract Here (for archives), Compress (for files/folders)
- **Delete hotkey** — press `Delete` to move selected files to trash
- **Permanent delete for large files** — files totaling >= 1 GB trigger a confirmation dialog and bypass trash
- **Keyboard shortcuts** — `Escape` to clear selection, `Mod+A` to select all
- **Progress streaming** — archive compress/extract operations now emit real-time progress events to the UI toast
- **Non-blocking archive operations** — compress/extract run on Tokio's thread pool via `spawn_blocking`, keeping the UI responsive

### Changed
- **Component split** — `FileGrid` refactored into smaller components: `FileGridToolbar`, `FileGridView`, `FileListView`, `TrashView` for easier debugging
- **Mutation pattern** — all async actions (delete, paste, rename, create, restore, purge, empty trash, compress, extract) use `@tanstack/svelte-query` mutations with `svelte-sonner` toast feedback
- **Hotkey library** — migrated to `@tanstack/svelte-hotkeys` for keyboard shortcut management
- **State management** — shared selection state extracted to `stores/selection.svelte.ts`
- **Context menu** — fixed Svelte 5 reactivity by using plain `$state` variables instead of `$state({...methods})` objects

### Fixed
- **Context menu crash** — `$state()` with methods inside helper functions caused infinite re-render loops; fixed by inlining state in component
- **Type errors** — `ComponentType` from `svelte` replaced with `ComponentType` from `svelte/component` for lucide-svelte icon compatibility
- **Accessibility warnings** — trash list rows now have `role="button"`, `tabindex`, and `onkeydown` handlers
