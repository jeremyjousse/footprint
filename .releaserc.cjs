module.exports = {
  branches: ["main"],
  repositoryUrl: "git@github.com:jeremyjousse/footprint.git",
  tagFormat: "${version}",
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    ["@semantic-release/npm", { npmPublish: false }],
    [
      "@semantic-release/exec",
      {
        prepareCmd:
          'echo "NEXT_RELEASE=${nextRelease.version}" >> $GITHUB_ENV && tomato set package.version ${nextRelease.version} src-tauri/Cargo.toml && cd src-tauri && cargo generate-lockfile && jq \'.version =  "${nextRelease.version}"\' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp && mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json',
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: [
          "CHANGELOG.md",
          "package.json",
          "package-lock.json",
          "src-tauri/Cargo.lock",
          "src-tauri/Cargo.toml",
          "src-tauri/tauri.conf.json",
        ],
        message: "chore(release): ${nextRelease.version} [skip ci]",
      },
    ],
    "@semantic-release/github",
  ],
};
