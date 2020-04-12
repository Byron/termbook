class Termbook < Formula
  # ---> DO NOT EDIT <--- (this file was generated from ./pkg/brew/termbook.rb.in
  version 'v1.4.5'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "c0f7f7c26b173acdb9f3399a15140ab728c7a3361eb274400355fb2b4bc1a56b"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "87bcf3f800b97f9e6a677022ec045f9152c8dcfc5b3246fb665396f097978b7f"
  end

  def install
    bin.install "termbook"
  end
end
