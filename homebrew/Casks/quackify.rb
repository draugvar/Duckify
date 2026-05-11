cask "quackify" do
  version "1.1.9"
  sha256 "PLACEHOLDER_SHA256"

  url "https://github.com/draugvar/Duckify/releases/download/v#{version}/quackify-macos-universal.tar.gz"

  name "Quackify"
  desc "DuckDuckGo Email Converter"
  homepage "https://github.com/draugvar/Duckify"

  app "Quackify.app"

  zap trash: [
    "~/Library/Application Support/Quackify",
    "~/Library/Preferences/com.draugvar.quackify.plist",
  ]
end
