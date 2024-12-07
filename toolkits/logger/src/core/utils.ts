import type { LogMessage } from "../types"

// -- 格式化消息
export function formatMessage(message: LogMessage): string {
	if (message instanceof Error) {
		return message.stack || message.message
	}

	if (typeof message === "object") {
		return JSON.stringify(message, null, 2)
	}

	return String(message)
}

// -- 格式化时间戳
export function formatTimestamp(): string {
	return new Date().toLocaleTimeString()
}

// -- 检查是否为空数组
export function isEmptyArray<T>(arr: T[]): boolean {
	return Array.isArray(arr) && arr.length === 0
}
