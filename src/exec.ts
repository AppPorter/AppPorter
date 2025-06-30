import { invoke } from '@tauri-apps/api/core'

export async function exec<T = unknown>(
  name: string,
  params?: Record<string, unknown>
): Promise<T> {
  const result = await invoke<string>('exec', {
    cmd: {
      name: name,
      ...params,
    },
  })
  return JSON.parse(result)
}
