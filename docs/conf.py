project = 'Mezzotint'
copyright = '2023, Bo Maryniuk'
author = 'Bo Maryniuk'
version = "0.1"
release = "Pre-release"

extensions = ["myst_parser"]
source_suffix = {
    '.rst': 'restructuredtext',
    '.txt': 'restructuredtext',
    '.md': 'markdown',
}

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

html_show_sourcelink = False
html_static_path = ['_static']
