#!/usr/bin/env node

const { execFileSync } = require("child_process");
const { existsSync, mkdirSync, chmodSync, createWriteStream } = require("fs");
const path = require("path");
const https = require("https");
const { execSync } = require("child_process");

const PLATFORM_MAP = {
  "linux-x64": {
    pkg: "@narusenia/pencil-analyzer-linux-x64",
    target: "x86_64-unknown-linux-gnu",
  },
  "linux-arm64": {
    pkg: "@narusenia/pencil-analyzer-linux-arm64",
    target: "aarch64-unknown-linux-gnu",
  },
  "darwin-x64": {
    pkg: "@narusenia/pencil-analyzer-darwin-x64",
    target: "x86_64-apple-darwin",
  },
  "darwin-arm64": {
    pkg: "@narusenia/pencil-analyzer-darwin-arm64",
    target: "aarch64-apple-darwin",
  },
  "win32-x64": {
    pkg: "@narusenia/pencil-analyzer-win32-x64",
    target: "x86_64-pc-windows-msvc",
  },
};

const VERSION = require("../package.json").version;
const REPO = "NaruseNia/pencil_analyzer";

function getPlatformKey() {
  const key = `${process.platform}-${process.arch}`;
  if (!PLATFORM_MAP[key]) {
    console.error(
      `Error: Unsupported platform ${process.platform}-${process.arch}.`
    );
    console.error(`Supported: ${Object.keys(PLATFORM_MAP).join(", ")}`);
    process.exit(1);
  }
  return key;
}

function tryResolvePkg(key) {
  const { pkg } = PLATFORM_MAP[key];
  const binary =
    process.platform === "win32" ? "pencil_analyzer.exe" : "pencil_analyzer";
  try {
    const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
    const binPath = path.join(pkgDir, "bin", binary);
    if (existsSync(binPath)) return binPath;
  } catch {
    // optional dependency not installed
  }
  return null;
}

function downloadUrl(key) {
  const { target } = PLATFORM_MAP[key];
  const tag = `v${VERSION}`;
  const ext = process.platform === "win32" ? "zip" : "tar.gz";
  return `https://github.com/${REPO}/releases/download/${tag}/pencil_analyzer-${tag}-${target}.${ext}`;
}

function cacheBinDir() {
  const dir = path.join(__dirname, ".cache");
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  return dir;
}

function cachedBinaryPath() {
  const binary =
    process.platform === "win32" ? "pencil_analyzer.exe" : "pencil_analyzer";
  return path.join(cacheBinDir(), binary);
}

function fetch(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          return fetch(res.headers.location).then(resolve, reject);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
        }
        resolve(res);
      })
      .on("error", reject);
  });
}

async function downloadBinary(key) {
  const url = downloadUrl(key);
  const binPath = cachedBinaryPath();

  if (existsSync(binPath)) return binPath;

  console.error(`Downloading pencil_analyzer v${VERSION}...`);

  const archivePath = path.join(
    cacheBinDir(),
    process.platform === "win32" ? "archive.zip" : "archive.tar.gz"
  );

  const res = await fetch(url);
  await new Promise((resolve, reject) => {
    const out = createWriteStream(archivePath);
    res.pipe(out);
    out.on("finish", resolve);
    out.on("error", reject);
  });

  // Extract
  if (process.platform === "win32") {
    execSync(
      `powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${cacheBinDir()}' -Force"`,
      { stdio: "ignore" }
    );
  } else {
    execSync(`tar xzf "${archivePath}" -C "${cacheBinDir()}"`, {
      stdio: "ignore",
    });
  }

  if (!process.platform === "win32") {
    chmodSync(binPath, 0o755);
  }

  // Clean up archive
  try {
    require("fs").unlinkSync(archivePath);
  } catch {}

  if (!existsSync(binPath)) {
    console.error("Error: Failed to extract binary from release archive.");
    process.exit(1);
  }

  if (process.platform !== "win32") {
    chmodSync(binPath, 0o755);
  }

  console.error("Done.");
  return binPath;
}

async function main() {
  const key = getPlatformKey();

  // Try optional dependency first
  let binary = tryResolvePkg(key);

  // Fallback: download from GitHub Releases
  if (!binary) {
    try {
      binary = await downloadBinary(key);
    } catch (e) {
      console.error(`Error: Failed to download binary: ${e.message}`);
      console.error(
        `You can manually download from: https://github.com/${REPO}/releases`
      );
      process.exit(1);
    }
  }

  try {
    execFileSync(binary, process.argv.slice(2), { stdio: "inherit" });
  } catch (e) {
    if (e.status !== undefined) {
      process.exit(e.status);
    }
    throw e;
  }
}

main();
