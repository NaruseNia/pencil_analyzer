#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");

const PLATFORM_MAP = {
  "linux-x64": "@narusenia/pencil-analyzer-linux-x64",
  "linux-arm64": "@narusenia/pencil-analyzer-linux-arm64",
  "darwin-x64": "@narusenia/pencil-analyzer-darwin-x64",
  "darwin-arm64": "@narusenia/pencil-analyzer-darwin-arm64",
  "win32-x64": "@narusenia/pencil-analyzer-win32-x64",
};

function getBinaryPath() {
  const key = `${process.platform}-${process.arch}`;
  const pkg = PLATFORM_MAP[key];

  if (!pkg) {
    console.error(
      `Error: Unsupported platform ${process.platform}-${process.arch}.`
    );
    console.error(`Supported: ${Object.keys(PLATFORM_MAP).join(", ")}`);
    process.exit(1);
  }

  const binary = process.platform === "win32" ? "pencil_analyzer.exe" : "pencil_analyzer";

  try {
    const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
    return path.join(pkgDir, "bin", binary);
  } catch {
    console.error(`Error: Could not find package '${pkg}'.`);
    console.error("This usually means the optional dependency was not installed.");
    console.error("Try reinstalling: npm install @narusenia/pencil-analyzer");
    process.exit(1);
  }
}

const binary = getBinaryPath();

try {
  execFileSync(binary, process.argv.slice(2), { stdio: "inherit" });
} catch (e) {
  if (e.status !== undefined) {
    process.exit(e.status);
  }
  throw e;
}
