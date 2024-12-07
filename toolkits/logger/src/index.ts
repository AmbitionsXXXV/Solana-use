export * from "./constants"
export * from "./core/Logger"
export * from "./types"

// -- 导出默认实例
import { Logger } from "./core/Logger"
export const logger = new Logger()
