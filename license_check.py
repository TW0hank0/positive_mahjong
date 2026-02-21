import subprocess
import sys


def main():
    command = [
        "addlicense",
        "-check",
        "-f",
        "addlicense.template",
        "-ignore",
        ".git/**",
        "-ignore",
        "**/.venv/**",
        "-ignore",
        "dist/**",
        "-ignore",
        "pkg/**",
        "-ignore",
        "target/**",
        "-ignore",
        "build/**",
        "-ignore",
        "**/__pycache__/**",
        "-ignore",
        "**/*.lock",
        "-ignore",
        ".python-version",
        "-ignore",
        "**/*.png",
        "-ignore",
        "**/*.kra",
        "-ignore",
        "**/*.ttf",
        "-ignore",
        "assets/",
        "-ignore",
        "**/*.json",
        "-ignore",
        "ThirdPartyLicense-Rust.html",
        "-ignore",
        "ThirdPartyLicense-Python.html",
        "-ignore",
        "ThirdPartyLicense.html",
        "-ignore",
        "src/licenses_rust.rs",
        "-ignore",
        "src/licenses.rs",
        "-ignore",
        "src/licenses_python.rs",
        "-ignore",
        "**/*.icon",
        "-ignore",
        "**/*.sh",
        ".",
    ]
    print(" ".join(command))
    print("-" * 10)
    subprocess.run(
        command,
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
        timeout=180,
    )


main()
