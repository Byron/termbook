class Termbook < Formula
  version '1.3.0'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "c132a01dae3ff7fe5b2a517843e513c798aadf92fbd6e0b4042a38a45111c8f5"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "0370bec25befd954aa254f60f98338771b528fccb36fb4b9f0f13bd6bda32d69"
  end

  def install
    bin.install "termbook"
  end
end
