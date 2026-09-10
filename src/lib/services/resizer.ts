import { Channel, convertFileSrc, invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { FileItem, ProcessProgress, ProcessResult, ResizeBackendOptions } from '$lib/types';

export const SUPPORTED_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.heic', '.webp'];

export function isImageFile(fileName: string): boolean {
  const lower = fileName.toLowerCase();
  return SUPPORTED_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

export function getParentDirectory(filePath: string): string {
  const lastSlash = Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\'));
  return lastSlash > 0 ? filePath.substring(0, lastSlash) : '';
}

export function createFileInputItem(filePath: string): FileItem {
  const name = filePath.split(/[/\\]/).pop() || filePath;
  return {
    id: crypto.randomUUID(),
    name,
    path: filePath,
    previewUrl: convertFileSrc(filePath),
  };
}

export async function pickImageFiles(): Promise<string[]> {
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: 'Images (JPG, PNG, HEIC, WEBP)',
        extensions: ['jpg', 'jpeg', 'png', 'heic', 'webp'],
      },
    ],
  });

  if (!selected) return [];
  return Array.isArray(selected) ? selected : [selected];
}

export async function pickOutputDirectory(): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select Destination Folder',
  });

  if (selected && typeof selected === 'string') {
    return selected;
  }
  return null;
}

export async function openOutputDirectory(dirPath: string): Promise<void> {
  if (!dirPath) return;
  await invoke('open_folder', { path: dirPath });
}

export async function executeBatchResize(
  paths: string[],
  options: ResizeBackendOptions,
  onProgress?: (progress: ProcessProgress) => void
): Promise<ProcessResult[]> {
  const channel = new Channel<ProcessProgress>();

  if (onProgress) {
    channel.onmessage = onProgress;
  }

  return await invoke<ProcessResult[]>('resize_images', {
    paths,
    options,
    onProgress: channel,
  });
}
