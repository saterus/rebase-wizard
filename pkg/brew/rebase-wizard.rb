class RebaseWizard < Formula
  desc "Let the Wizard guide you through the darkest of gituations"
  homepage "https://github.com/saterus/rebase-wizard"
  url "https://github.com/saterus/rebase-wizard/archive/v0.0.1-alpha4.tar.gz"
  sha256 "7c59dc446d3501a6197b327c1b6212dd26c9977f0a88c64389bcc6ef1f09f182"
  head "git@github.com:saterus/rebase-wizard.git"

  bottle do
    root_url "https://github.com/saterus/rebase-wizard/releases/download/v0.0.1-alpha4"
    cellar :any_skip_relocation
    sha256 "ec5a51c264087e468fac695e8477af0c3f816cd144e9f542a56060d9b181f583" => :mojave
  end

  depends_on "rust" => [:build]

  def install
    (buildpath/"src/github.com/saterus").mkpath
    ln_s buildpath, buildpath/"src/github.com/saterus/rebase-wizard"
    system "cargo", "install", "-vv", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match(/Wizard Secrets/, pipe_output("#{bin}/rebase-wizard help"))
  end
end
