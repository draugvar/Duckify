cask "quackify" do
  version "1.1.9"
  sha256 "REPLACE_WITH_SHA256_FROM_PACKAGE_SCRIPT"

  url "https://github.com/draugvar/Duckify/releases/download/v#{version}/quackify-macos-universal.tar.gz"

  name "Quackify"
  desc "DuckDuckGo Email Converter"
  homepage "https://github.com/draugvar/Duckify"

  livecheck do
    url :url
    strategy :github_latest
  end

  app "Quackify.app"
end
