// -- 日志级别
export type LogLevel = "success" | "info" | "warn" | "error" | "debug"

// -- 日志消息类型
export type LogMessage = string | number | boolean | object | Error

// -- 日志配置
export interface LogConfig {
	timestamp?: boolean
	prefix?: string
	level?: LogLevel
	disabled?: boolean
}

// -- 日志主题配置
type ChalkColor =
	| "red"
	| "green"
	| "yellow"
	| "blue"
	| "magenta"
	| "cyan"
	| "gray"

export type LogTheme = {
	readonly [key in LogLevel]: {
		icon: string
		color: ChalkColor
	}
}
