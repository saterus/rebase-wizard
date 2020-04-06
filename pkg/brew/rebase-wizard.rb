class RebaseWizard < Formula
  desc "Let the Wizard guide you through the darkest of gituations"
  homepage "https://github.com/saterus/rebase-wizard"
  url "https://github.com/saterus/rebase-wizard/archive/v0.99.2.tar.gz"
  sha256 "aa6b4fd58b30c7b0e2e9a7211075b8245f735ba9193bebafc011035a730dbf5b"
  head "git@github.com:saterus/rebase-wizard.git"

  bottle do
    root_url "https://github.com/saterus/rebase-wizard/releases/download/v0.99.2/"
    cellar :any_skip_relocation
    sha256 "5fac691cdc0cb042ea83612ac859f294f1958931644db1d9ffcb1767e912452e" => :mojave
  end

  depends_on "rust" => [:build]

  def install
    (buildpath/"src/github.com/saterus").mkpath
    ln_s buildpath, buildpath/"src/github.com/saterus/rebase-wizard"
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match(/Wizard Secrets/, pipe_output("#{bin}/rebase-wizard help"))
  end
end
