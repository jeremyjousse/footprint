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
          'echo "NEXT_RELEASE=${nextRelease.version}" >> $GITHUB_ENV && tomato set package.version ${nextRelease.version} src-tauri/Cargo.toml && cd src-tauri && cargo generate-lockfile',
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
        ],
        message: "chore(release): ${nextRelease.version} [skip ci]",
      },
    ],
    "@semantic-release/github",
  ],
};
