import tomllib
import zipfile
import os
import platform


def main():
    info_file = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "Cargo.toml"
    )
    with open(info_file, "rb") as f:
        project_info = tomllib.load(f)
    version = project_info["workspace"]["package"]["version"]
    #
    include_files = []
    target_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        "target",
        # "x86_64-pc-windows-msvc",
        "release",
    )
    for file in os.listdir(target_path):
        full_file_path = os.path.join(target_path, file)
        if os.path.isfile(full_file_path) is True:
            match platform.system():
                case "Linux":
                    if len(file.split(".")) == 1:
                        include_files.append(full_file_path)
                case "Windows":
                    if (file.split(".")[1] == "exe") and (
                        len(file.split(".")) > 1
                    ):
                        include_files.append(full_file_path)
    #
    # launcher_path = os.path.abspath(
    #     os.path.join(
    #         os.path.dirname(os.path.dirname(__file__)),
    #         "dist",
    #         "ptb_launcher",
    #     )
    # )
    match platform.system():
        case "Linux":
            pf = "linux"
        case "Windows":
            pf = "windows"
            # launcher_path = launcher_path + ".exe"
        case _:
            pf = "unknown"
    # include_files.append(launcher_path)
    zip_file_name = f"positive_mahjong_v{version}_{pf}.zip"
    with zipfile.ZipFile(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            zip_file_name,
        ),
        mode="w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zipf:
        for file in include_files:
            zipf.write(file, arcname=os.path.basename(file))
    print(zip_file_name)


if __name__ == "__main__":
    main()
