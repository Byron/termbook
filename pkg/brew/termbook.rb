class Termbook < Formula
  version '1.2.4'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "7100f663d05ddd606c35ea66440a67bf6fedd9439079921b4bb557f3ca11f951"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "cd18d34b8b3b9ff9bc0438e2fe07954256b99d8971a17869d9467d19b769a62c"
  end

  def install
    bin.install "termbook"
  end
end
