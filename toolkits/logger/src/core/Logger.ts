import chalk from "chalk"
import { DEFAULT_CONFIG, DEFAULT_THEME } from "../constants"
import type { LogConfig, LogLevel, LogMessage, LogTheme } from "../types"
import { formatMessage, formatTimestamp, isEmptyArray } from "./utils"

export class Logger {
	private config: LogConfig
	private theme: LogTheme

	constructor(config: Partial<LogConfig> = {}, theme: Partial<LogTheme> = {}) {
		this.config = { ...DEFAULT_CONFIG, ...config }
		this.theme = { ...DEFAULT_THEME, ...theme }
	}

	private log(level: LogLevel, message: LogMessage): void {
		if (this.config.disabled) return

		const parts: string[] = []
		const { icon, color } = this.theme[level]

		// -- 添加时间戳
		if (this.config.timestamp) {
			parts.push(chalk.gray(formatTimestamp()))
		}

		// -- 添加前缀
		if (this.config.prefix) {
			parts.push(this.config.prefix)
		}

		// -- 添加图标和级别
		parts.push(`${icon} ${chalk[color].bold(`[${level.toUpperCase()}]`)}`)

		// -- 添加消息
		parts.push(formatMessage(message))

		console[level === "error" ? "error" : "log"](parts.join(" "))
	}

	success(message: LogMessage): void {
		this.log("success", message)
	}

	info(message: LogMessage): void {
		this.log("info", message)
	}

	warn(message: LogMessage): void {
		this.log("warn", message)
	}

	error(message: LogMessage): void {
		this.log("error", message)
	}

	debug(message: LogMessage): void {
		this.log("debug", message)
	}

	table<T extends object>(data: T[]): void {
		if (isEmptyArray(data)) {
			this.info("空数据集")
			return
		}
		console.table(data)
	}

	configure(options: Partial<LogConfig>): void {
		this.config = { ...this.config, ...options }
	}

	setTheme(theme: Partial<LogTheme>): void {
		this.theme = { ...this.theme, ...theme }
	}
}
