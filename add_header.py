import os
import glob

def add_frontmatter_to_files():
    """
    Adds a frontmatter header to all markdown files in the specified directory
    and its subdirectories, if they don't already have one.
    """
    path = "/home/nicole/Documents/tryptamine/blog-content/content/**/*.md"
    files = glob.glob(path, recursive=True)

    for file_path in files:
        with open(file_path, 'r+') as f:
            content = f.read()
            if not content.startswith("+++"):
                # Generate title from filename
                filename = os.path.basename(file_path)
                title = os.path.splitext(filename)[0].replace("_", " ").replace("-", " ").title()

                # Create frontmatter
                frontmatter = f"""+++
title = "{title}"
date = 2025-01-01

[extra]
author = "Nicole Venner"
+++

"""
                # Write new content
                f.seek(0, 0)
                f.write(frontmatter + content)
                print(f"Added header to: {file_path}")

if __name__ == "__main__":
    add_frontmatter_to_files()
