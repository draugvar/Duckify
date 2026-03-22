cask "duckify" do
  version "1.1.2"
  sha256 "PLACEHOLDER_SHA256"

  url "https://github.com/draugvar/Duckify/releases/download/v#{version}/duckify-macos-universal.tar.gz"

  name "Duckify"
  desc "DuckDuckGo-powered browser in a native macOS wrapper"
  homepage "https://github.com/draugvar/Duckify"

  app "Duckify.app"

  zap trash: [
    "~/Library/Application Support/duckify",
    "~/Library/Preferences/duckify.plist",
  ]
end
