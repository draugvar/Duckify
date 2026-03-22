cask "duckify" do
  version "1.1.1"
  sha256 "REPLACE_WITH_SHA256_FROM_PACKAGE_SCRIPT"

  url "https://github.com/draugvar/DuckDuckUI/releases/download/v#{version}/Duckify-#{version}.dmg"

  name "Duckify"
  desc "Convert any email to a duck.com alias"
  homepage "https://github.com/draugvar/DuckDuckUI"

  livecheck do
    url :url
    strategy :github_latest
  end

  app "Duckify.app"
end
