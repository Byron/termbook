class Termbook < Formula
  # ---> DO NOT EDIT <--- (this file was generated from ./pkg/brew/termbook.rb.in
  version '1.4.1'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0b0782098ffb96209a02bd889fcdaf1c100f131889d67bf6ff16d9f849df4681"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "ddd3b78051a12f4d95e816a0e479b3d5dbaa3cb4579e16d7f283c78976993feb"
  end

  def install
    bin.install "termbook"
  end
end
