class Termbook < Formula
  # ---> DO NOT EDIT <--- (this file was generated from ./pkg/brew/termbook.rb.in
  version '1.4.0'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "4b7c0a0e68cf153e0c49dba903592c537cd6eb6cbad41230a2d103a08b1a5cb0"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "b981b557e2c5ca703893a0bedb0bc9d09bf8ca7fb1dcd117876817b4914421c4"
  end

  def install
    bin.install "termbook"
  end
end
