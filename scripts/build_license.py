import subprocess
import sys


def main():
    print("-" * 10, "cargo-about", "-" * 10)
    command = [
        "cargo",
        "about",
        "generate",
        "--output-file",
        "auto_generated/ThirdPartyLicense-Rust.html",
        "--threshold",
        "1.0",
        "templates/about_html.hbs",
    ]
    print(f"Run Command: {' '.join(command)}")
    subprocess.run(
        command,
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    command2 = [
        "cargo-about",
        "generate",
        "--threshold",
        "1.0",
        "--output-file",
        "auto_generated/ThirdPartyLicense-Rust.json",
        "templates/about_json.hbs",
    ]
    print(f"Run Command: {' '.join(command2)}")
    subprocess.run(
        command2,
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    command3 = [
        "cargo-about",
        "generate",
        "--output-file",
        "auto_generated/ThirdPartyLicense-Rust.md",
        "--threshold",
        "1.0",
        "templates/about_markdown.hbs",
    ]
    print(f"Run Command: {' '.join(command3)}")
    subprocess.run(
        command3,
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )


if __name__ == "__main__":
    main()
