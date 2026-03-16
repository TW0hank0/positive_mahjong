import subprocess
import sys
import os


def main():
    orig_cwd = os.getcwd()
    os.chdir(os.path.abspath(os.path.join(__file__, "..", "..", "pmj_client")))
    command = [
        "wasm-pack",
        "build",
        "--release",
        "--target",
        "web",
        "--bin",
        "pmj_client",
    ]
    print("Run Command:" + " ".join(command))
    subprocess.run(
        command,
        stdin=sys.stdin,
        stdout=sys.stdout,
        stderr=sys.stderr,
        check=True,
    )
    os.chdir(orig_cwd)


if __name__ == "__main__":
    main()
