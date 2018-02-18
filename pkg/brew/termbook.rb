class Termbook < Formula
  version '1.2.3'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "2debbbcf907196f8b4dc23235462fb1298ba9880687a5b256d2ef68d1ea999eb"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "830a90aa8f61f7d2cc7778771ea6405b3e04b1e3b5db24742c7b708e8f3ada38"
  end

  def install
    bin.install "termbook"
  end
end
