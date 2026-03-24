// Mock @tauri-apps/plugin-clipboard-manager
let clipboardContent = "";

export async function writeText(text: string): Promise<void> {
  clipboardContent = text;
}

export async function readText(): Promise<string> {
  return clipboardContent;
}

export function __getClipboard(): string {
  return clipboardContent;
}
