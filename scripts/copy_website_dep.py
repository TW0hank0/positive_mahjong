import os
import shutil

import util


def main():
    os.chdir(os.path.join(os.path.dirname(os.path.dirname(__file__)), "docs"))
    util.run_cmd(["mdbook", "build"])
    shutil.copytree(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "docs", "book"),
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "website", "docs"),
    )
    shutil.copy2(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "assets", "icon", "icon.png"),
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "website", "icon.png"),
    )
    shutil.copy2(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "assets", "icon", "icon.svg"),
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "website", "icon.svg"),
    )
    shutil.copy2(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "LICENSE"),
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "website", "LICENSE"),
    )


if __name__ == "__main__":
    main()
