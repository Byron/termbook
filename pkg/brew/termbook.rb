class Termbook < Formula
  version '1.2.2'
  desc "A runner for `mdbooks` to keep your documentation tested."
  homepage "https://github.com/Byron/termbook"

  if OS.mac?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "45cc4480e9a52ffdc9064d29ae402e348a934b72f16857dfca7c4df4767cbec4"
  elsif OS.linux?
      url "https://github.com/Byron/termbook/releases/download/#{version}/termbook-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "21a37c7311e8addcadb56e9112d755a932dc1ed5900a495c14bf1d484f1703e3"
  end

  def install
    bin.install "termbook"
  end
end
