# /// script
# dependencies = [
#     "mistune==3.3.4"
# ]
# ///
import os
import shutil
import mistune


def main():
    website_root_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "website"
    )
    nav_template_path = os.path.join(website_root_path, "nav.html.template")
    with open(nav_template_path, "r", encoding="utf-8") as f:
        nav_template_content = f.read()
    ignored: list[str] = ["docs", ".git", "__pycache__", "__pypy_cache__"]
    build_root = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "website_build"
    )
    shutil.copytree(website_root_path, build_root, dirs_exist_ok=True)
    process_dir(
        dir_path=build_root,
        ignored=ignored,
        nav_template=nav_template_content,
        website_root_path=build_root,
    )


def process_dir(
    dir_path: str, ignored: list[str], nav_template: str, website_root_path: str
):
    for dir in os.listdir(dir_path):
        full_dir_path = os.path.join(dir_path, dir)
        if dir in ignored:
            continue
        else:
            if os.path.isfile(full_dir_path) is True:
                if dir.endswith(".html") is True:
                    print(f"Found html file: {full_dir_path}")
                    with open(full_dir_path, "r", encoding="utf-8") as f:
                        orig_content = f.read()
                    new_content = replace_var(
                        full_dir_path, nav_template, website_root_path
                    )
                    if new_content != orig_content:
                        with open(full_dir_path, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        print(f"Wrote html file: {full_dir_path}")
            elif os.path.isdir(full_dir_path) is True:
                process_dir(
                    dir_path=full_dir_path,
                    ignored=ignored,
                    nav_template=nav_template,
                    website_root_path=website_root_path,
                )
            else:
                print(f"???: isfile=False, isdir=False, dir={dir}")


def replace_var(
    replace_html_path: str, nav_template: str, website_root_path: str
) -> str:
    orig_work_dir = os.getcwd()
    template_vars: dict[str, str] = {}
    os.chdir(website_root_path)
    rel_path = os.path.relpath(
        os.path.dirname(replace_html_path), start=website_root_path
    )
    template_vars["VAR_ROOT_DIR"] = rel_path
    new_template = nav_template
    for key in list(template_vars.keys()):
        key_fixed_name = "{{$" + key + "$}}"
        if key_fixed_name in new_template:
            new_template = new_template.replace(key_fixed_name, template_vars[key])
            print(f"Replaced `{key}` in template.")
    html_vars: dict[str, str] = {}
    # print(f"new_template={new_template}")
    html_vars["VAR_NAV"] = new_template
    with open(os.path.join(os.path.dirname(os.path.dirname(__file__)), "LICENSE"), "r", encoding="utf-8") as f:
        html_vars["VAR_LICENSE"] = f.read()
    with open(os.path.join(os.path.dirname(os.path.dirname(__file__)), "auto_generated", "ThirdPartyLicense-Rust.md"), "r", encoding="utf-8") as f:
        t=f.read()
        html_vars["VAR_THIRD_PARTY_LICENSE_RUST_MD"] = t
        html_vars["VAR_THIRD_PARTY_LICENSE_RUST_MD_TO_HTML"] = str(mistune.html(t))
    #
    with open(replace_html_path, "r", encoding="utf-8") as f:
        new_html_content = f.read()
    for key in list(html_vars.keys()):
        key_fixed_name = "{{$" + key + "$}}"
        if key_fixed_name in new_html_content:
            new_html_content = new_html_content.replace(key_fixed_name, html_vars[key])
            print(f"Replaced `{key}` in file: {replace_html_path}")
    os.chdir(orig_work_dir)
    # print(f"new_html_content={new_html_content}")
    return new_html_content


if __name__ == "__main__":
    main()
