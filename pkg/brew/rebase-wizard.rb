class RebaseWizard < Formula
  desc "Let the Wizard guide you through the darkest of gituations"
  homepage "https://github.com/saterus/rebase-wizard"
  url "https://github.com/saterus/rebase-wizard/archive/v1.0.0.tar.gz"
  sha256 "bb57a2080fa37e0bf2eb1250ba60690e023f1581a5426ee95da05a7d07743f4a"
  head "git@github.com:saterus/rebase-wizard.git"

  bottle do
    root_url "https://github.com/saterus/rebase-wizard/releases/download/v1.0.0/"
    cellar :any_skip_relocation
    sha256 "d11c2033dd2887ac25185f8fcf1334b16e1b1e7379876b0bdcca64703f481559" => :mojave
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
