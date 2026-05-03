import os
import subprocess


def main():
    packages = [
        "pmj_client",
        "pmj_server",
        "pmj_shared",
        "pmj_gamemodes",
    ]
    for pkg in packages:
        subprocess.run(
            ["cargo", "msrv", "find"],
            cwd=os.path.join(
                os.path.dirname(os.path.dirname(os.path.dirname(__file__))), pkg
            ),
        )
        subprocess.run(
            ["cargo", "msrv", "verify"],
            cwd=os.path.join(
                os.path.dirname(os.path.dirname(os.path.dirname(__file__))), pkg
            ),
        )


main()
