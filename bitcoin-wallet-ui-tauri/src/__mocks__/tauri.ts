// Mock @tauri-apps/api/core
const mockResponses: Record<string, any> = {};

export function __setMockResponse(command: string, response: any) {
  mockResponses[command] = response;
}

export function __clearMocks() {
  Object.keys(mockResponses).forEach((key) => delete mockResponses[key]);
}

export async function invoke<T>(command: string, args?: any): Promise<T> {
  if (command in mockResponses) {
    const response = mockResponses[command];
    if (response instanceof Error) throw response;
    if (typeof response === "function") return response(args);
    return response as T;
  }
  throw new Error(`No mock response set for command: ${command}`);
}
