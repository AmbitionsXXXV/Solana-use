import type { LogTheme } from "./types"

// -- 默认主题
export const DEFAULT_THEME: LogTheme = {
	success: { icon: "✨", color: "green" },
	info: { icon: "ℹ️", color: "blue" },
	warn: { icon: "⚠️", color: "yellow" },
	error: { icon: "❌", color: "red" },
	debug: { icon: "🔍", color: "gray" },
}

// -- 默认配置
export const DEFAULT_CONFIG = {
	timestamp: true,
	prefix: "🚀",
	level: "info" as const,
	disabled: false,
}
