// Example file preview plugin
(function() {
  'use strict';

  // Register preview handlers for supported file types
  const supportedTypes = ['txt', 'md', 'json'];

  async function previewFile(filePath) {
    try {
      const content = await pluginAPI.readFile(filePath);

      // Create preview container
      const preview = document.createElement('div');
      preview.className = 'file-preview';
      preview.style.cssText = 'padding: 1rem; font-family: monospace; white-space: pre-wrap; overflow: auto;';

      // Check file extension
      const ext = filePath.split('.').pop()?.toLowerCase();

      if (ext === 'json') {
        try {
          const formatted = JSON.stringify(JSON.parse(content), null, 2);
          preview.textContent = formatted;
        } catch {
          preview.textContent = content;
        }
      } else {
        preview.textContent = content;
      }

      // Clear previous preview and show new one
      document.body.innerHTML = '';
      document.body.appendChild(preview);

      pluginAPI.showToast(`Previewing: ${filePath}`);
    } catch (error) {
      pluginAPI.showToast(`Error previewing file: ${error.message}`);
    }
  }

  async function showFileInfo() {
    const files = await pluginAPI.getSelectedFiles();
    if (files.length > 0) {
      pluginAPI.showToast(`Selected: ${files.join(', ')}`);
    } else {
      pluginAPI.showToast('No files selected');
    }
  }

  // Make functions available globally for context menu
  window.previewFile = previewFile;
  window.showFileInfo = showFileInfo;

  console.log('My File Preview Plugin loaded!');
})();
