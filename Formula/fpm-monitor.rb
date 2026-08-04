# frozen_string_literal: true

# fpm-monitor Homebrew formula.
#
# This file is a TEMPLATE: the release workflow fills in the placeholders and
# attaches the generated `fpm-monitor.rb` to every GitHub release. Before first
# use, replace filoucrackeur/php-fpm-monitor with your GitHub repository, or install directly with:
#
#   brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
#
# To host it as a tap instead, move this file to:
#   <owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb

class FpmMonitor < Formula
  desc "PHP-FPM pools monitor (Rust port of fpm-monitor.c)"
  homepage "https://github.com/filoucrackeur/php-fpm-monitor"
  license "MIT"
  version "__VERSION__"

  url "__URL_INTEL__"
  sha256 "__SHA_INTEL__"

  on_arm do
    url "__URL_ARM__"
    sha256 "__SHA_ARM__"
  end

  def install
    bin.install "fpm-monitor"
  end

  test do
    assert_match "fpm-monitor", shell_output("#{bin}/fpm-monitor --help")
  end
end
