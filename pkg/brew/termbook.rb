class Termbook < Formula
  version '1.2.1'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}-release/termbook-#{version}-release-x86_64-apple-darwin.tar.gz"
      sha256 "b4937d3168ca86905cb0b2da3bf1240ff8e7b5f279761c0f337cf43e6b0ba83d"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}-release/termbook-#{version}-release-x86_64-unknown-linux-musl.tar.gz"
      sha256 "fa3188f27b6e18e1e6a2bad0c59c75bcde5aa362752124e094d28865929c3f7f"
  end

  def install
    bin.install "termbook"
  end
end