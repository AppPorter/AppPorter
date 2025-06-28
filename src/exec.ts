import { invoke } from '@tauri-apps/api/core'

export async function exec(name: string, params?: Record<string, unknown>): Promise<string> {
  const result = await invoke<string>('exec', {
    cmd: {
      name: name,
      ...params,
    },
  })
  return JSON.parse(result)
}
