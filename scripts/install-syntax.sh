#!/bin/sh
set -eu

# Resolve the repo root one level up from scripts/
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_DIR=$(CDPATH= cd -- "${SCRIPT_DIR}/.." && pwd)
SYNTAX_SRC="${REPO_DIR}/syntax_files"

if [ ! -d "${SYNTAX_SRC}" ]; then
    printf 'Error: Could not find %s\n' "${SYNTAX_SRC}" >&2
    exit 1
fi

installed_any=0

# Helper for Vim / Neovim directory layout
install_vim_like() {
    base_dir="$1"
    label="$2"
    vim_src="${SYNTAX_SRC}/lorebook.vim"

    if [ ! -f "${vim_src}" ]; then
        return
    fi

    mkdir -p "${base_dir}/syntax" "${base_dir}/ftdetect"
    cp "${vim_src}" "${base_dir}/syntax/lorebook.vim"

    ftdetect_file="${base_dir}/ftdetect/lorebook.vim"
    if [ ! -f "${ftdetect_file}" ]; then
        printf '%s\n' 'autocmd BufRead,BufNewFile *.lore setfiletype lorebook' > "${ftdetect_file}"
    fi

    printf 'Installed %s syntax highlighting -> %s\n' "${label}" "${base_dir}"
    installed_any=1
}

# 1. Vim (always install to ~/.vim or if vim is present)
if command -v vim >/dev/null 2>&1 || [ -d "${HOME}/.vim" ]; then
    install_vim_like "${HOME}/.vim" "Vim"
fi

# 2. Neovim
if command -v nvim >/dev/null 2>&1 || [ -d "${HOME}/.config/nvim" ]; then
    install_vim_like "${HOME}/.config/nvim" "Neovim"
fi

# 3. VS Code, Cursor, and Windsurf (TextMate extension)
if [ -f "${SYNTAX_SRC}/package.json" ] && [ -f "${SYNTAX_SRC}/lorebook.tmLanguage.json" ]; then
    for editor_dir in "${HOME}/.vscode/extensions" "${HOME}/.cursor/extensions" "${HOME}/.windsurf/extensions"; do
        if [ -d "${editor_dir}" ]; then
            target="${editor_dir}/lorebook-syntax"
            rm -rf "${target}"
            cp -R "${SYNTAX_SRC}" "${target}"
            printf 'Installed TextMate extension -> %s\n' "${target}"
            installed_any=1
        fi
    done
fi

if [ "${installed_any}" -eq 0 ]; then
    printf 'No supported editors detected. Manually copy files from %s\n' "${SYNTAX_SRC}"
fi

