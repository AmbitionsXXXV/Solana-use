import { promises as fs } from 'node:fs'; // 导入fs模块的promises API，用于异步文件操作
import path from 'node:path';

const create = async () => {
  const scripts = []

  for (let i = 0; i < 35; i++) {
    scripts.push({
      name: `miner-${i}`,
      script: `./mine_key${i}.sh`,
      interpreter: '/bin/bash',
    })
  }

  await fs.writeFile(
    path.resolve(__dirname, 'ecosystem.config.js'),
    `module.exports = {
      apps: ${JSON.stringify(scripts, null, 2)}
    };`,
  )
}

const createClaim = async () => {
  const scripts = []

  for (let i = 0; i < 35; i++) {
    scripts.push({
      name: `miner-${i}`,
      script: `./claim_key${i}.sh`,
      interpreter: '/bin/bash',
      restart_delay: 1000 * 60 * 30,
    })
  }

  await fs.writeFile(
    path.resolve(__dirname, 'ecosystem1.config.js'),
    `module.exports = {
      apps: ${JSON.stringify(scripts, null, 2)}
    };`,
  )
}

await create()
await createClaim()
