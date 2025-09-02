# ----------------------------------------------------------------
# NOTE: Setting shell does not work!
# For GitHub-actions we need "bash", but
# for Windows we need "sh".
# The solution is to ensure tasks are written with bash-shebang
# if they involve bash-syntax, e.g. 'if [[ ... ]] then else fi'.
# ----------------------------------------------------------------
# set shell := [ "bash", "-c" ]
_default:
    @- just --unsorted --list

menu:
    @- just --unsorted --choose

# ----------------------------------------------------------------
# Justfile
# Recipes for various workflows.
# ----------------------------------------------------------------

set dotenv-load := true
set positional-arguments := true

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# VARIABLES
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

PATH_ROOT := justfile_directory()
CURRENT_DIR := invocation_directory()
OS := if os_family() == "windows" { "windows" } else { "linux" }
PYVENV_ON := if os_family() == "windows" { ". .venv/Scripts/activate" } else { ". .venv/bin/activate" }
PYVENV := if os_family() == "windows" { "python" } else { "python3" }

# --------------------------------
# Macros
# --------------------------------

_clean-all-files path pattern:
    #!/usr/bin/env bash
    find {{path}} -type f -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    find {{path}} -type f -name "{{pattern}}" -exec rm {} \; 2> /dev/null
    exit 0;

_clean-all-folders path pattern:
    #!/usr/bin/env bash
    find {{path}} -type d -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    find {{path}} -type d -name "{{pattern}}" -exec rm -rf {} \; 2> /dev/null
    exit 0;

_check-tool tool name:
    #!/usr/bin/env bash
    success=false
    {{PYVENV_ON}} && {{tool}} --version >> /dev/null 2> /dev/null && success=true;
    {{PYVENV_ON}} && {{tool}} --help >> /dev/null 2> /dev/null && success=true;
    # NOTE: if exitcode is 251 (= help or print version), then render success.
    if [[ "$?" == "251" ]]; then success=true; fi
    # FAIL tool not installed
    if ( $success ); then
        echo -e "Tool \x1b[2;3m{{name}}\x1b[0m installed correctly.";
        exit 0;
    else
        echo -e "Tool \x1b[2;3m{{tool}}\x1b[0m did not work." >> /dev/stderr;
        echo -e "Ensure that \x1b[2;3m{{name}}\x1b[0m (-> \x1b[1mjust build\x1b[0m) installed correctly and system paths are set." >> /dev/stderr;
        exit 1;
    fi

_check-python-tool tool name:
    @just _check-tool "{{PYVENV}} -m {{tool}}" "{{name}}"

_rust_path_to_module path:
    #!/usr/bin/env bash
    path="{{path}}";
    path="${path%.*}";
    name="${path//[\/\\]/::}";
    name="${name#*::}";
    echo "${name}";
    exit 0;

_rust_path_to_test_module path:
    #!/usr/bin/env bash
    name="$(just _rust_path_to_module "{{path}}")";
    pref="";
    if [[ "$name" == *::* ]]; then
        parts_init="${name%::*}";
        name="${name##*::}";
        pref="${parts_init}::";
    fi
    if [[ "$name" != tests_* ]]; then
        name="tests_${name}";
    fi
    echo "${pref}${name}";
    exit 0;

# ----------------------------------------------------------------
# TARGETS
# ----------------------------------------------------------------

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: build
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

setup:
    @echo "TASK: SETUP"
    @- cp -n "templates/template.env" ".env"

build:
    @just build-venv
    @just build-requirements
    @just check-system-requirements
    @just build-compile

build-venv:
    @echo "create venv if not exists"
    @- ${PYTHON_PATH} -m venv .venv 2> /dev/null

build-requirements:
    @just build-requirements-basic
    @just build-requirements-dependencies

build-requirements-basic:
    @cargo update --verbose
    @cargo install --locked --force cargo-zigbuild
    @# cargo install --locked --force rustfmt
    @{{PYVENV_ON}} && {{PYVENV}} -m pip install --upgrade pip
    @{{PYVENV_ON}} && {{PYVENV}} -m pip install ruff uv

build-requirements-dependencies:
    @{{PYVENV_ON}} && {{PYVENV}} -m uv pip install \
        --exact \
        --strict \
        --compile-bytecode \
        --no-python-downloads \
        --requirements pyproject.toml
    @{{PYVENV_ON}} && {{PYVENV}} -m uv sync

build-compile module="${MAIN_MODULE}":
    @# cargo zigbuild --target-dir "target" --release --lib
    @cargo zigbuild --target-dir "target" --release --bin "${MAIN_MODULE}"

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: execution
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

run-py module="main" *args="":
    @{{PYVENV_ON}} && {{PYVENV}} src-py.{{module}} {{args}}

run-rust module="${MAIN_MODULE}" *args="":
    @just build-compile "{{module}}"
    @# "./target/release/{{module}}" {{args}}
    @cargo run --bin "{{module}}"

# --------------------------------
# TARGETS: development
# --------------------------------

dev *args:
    @echo "Not yet implemented"

# --------------------------------
# TARGETS: tests
# --------------------------------

tests:
    @just tests-unit

tests-logs log_path="logs":
    @just _reset-logs "{{log_path}}"
    @- just tests
    @just _display-logs

test-unit path *args:
    @cargo zigbuild --tests
    @echo "run unit tests in $( just _rust_path_to_test_module "{{path}}")"
    @cargo test --lib "$( just _rust_path_to_test_module "{{path}}")" {{args}} -- --nocapture
    @# echo "run unit tests in $( just _rust_path_to_module "{{path}}")"
    @# cargo test --lib "$( just _rust_path_to_module "{{path}}")" {{args}} -- --nocapture

test-unit-optimised path *args:
    @cargo zigbuild --tests --release
    @echo "run unit tests in $( just _rust_path_to_test_module "{{path}}")"
    @cargo test --lib "$( just _rust_path_to_test_module "{{path}}")" {{args}} -- --nocapture
    @# echo "run unit tests in $( just _rust_path_to_module "{{path}}")"
    @# cargo test --lib "$( just _rust_path_to_module "{{path}}")" {{args}} -- --nocapture

tests-unit *args:
    @just _reset-logs
    @cargo zigbuild --tests
    @cargo test --lib {{args}} -- --nocapture

tests-unit-optimised *args:
    @just _reset-logs
    @cargo zigbuild --tests --release
    @cargo test --lib {{args}} -- --nocapture

# --------------------------------
# TARGETS: prettify
# --------------------------------

prettify:
    @cargo fmt --verbose

prettify-dry:
    @echo "Not yet implemented"
    @# cargo fmt --verbose --check

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: clean
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

clean log_path="logs":
    @just clean-venv
    @just clean-basic "{{log_path}}"

clean-basic log_path="logs":
    @echo "All system artefacts will be force removed."
    @- just _clean-all-files "." ".DS_Store" 2> /dev/null
    @echo "All build artefacts will be force removed."
    @cargo clean
    @just _clean-all-files "." "*.rs.bk"
    @- rm -rf ".venv" 2> /dev/null
    @- rm -rf "target" 2> /dev/null

clean-venv:
    @echo "VENV will be removed."
    @- just _delete-if-folder-exists ".venv" 2> /dev/null

# --------------------------------
# TARGETS: logging, session
# --------------------------------

_clear-logs log_path="logs":
    @rm -rf "{{log_path}}" 2> /dev/null

_create-logs log_path="logs":
    @just _create-logs-part "debug" "{{log_path}}"
    @just _create-logs-part "out" "{{log_path}}"
    @just _create-logs-part "err" "{{log_path}}"

_create-logs-part part log_path="logs":
    @mkdir -p "{{log_path}}"
    @touch "{{log_path}}/{{part}}.log"

_reset-logs log_path="logs":
    @rm -rf "{{log_path}}" 2> /dev/null
    @just _create-logs "{{log_path}}"

_display-logs:
    @echo ""
    @echo "Content of logs/debug.log:"
    @echo "----------------"
    @echo ""
    @- cat logs/debug.log
    @echo ""
    @echo "----------------"

watch-logs n="10":
    @tail -f -n {{n}} logs/out.log

watch-logs-err n="10":
    @tail -f -n {{n}} logs/err.log

watch-logs-debug n="10":
    @tail -f -n {{n}} logs/debug.log

watch-logs-all n="10":
    @just watch-logs {{n}} &
    @just watch-logs-err {{n}} &
    @just watch-logs-debug {{n}} &

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# TARGETS: requirements
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

check-system:
    @echo "Operating System detected:  {{os_family()}}"
    @echo "Cargo command:              $( cargo --version )"
    @echo "Rustc command:              $( rustc --version )"
    @echo "Python command used:        ${PYTHON_PATH}"
    @echo "Python command for venv:    {{PYVENV}}"
    @echo "Python path for venv:       $( {{PYVENV_ON}} && which {{PYVENV}} )"
    @echo "Cargo Zigbuild:             $( cargo-zigbuild --version )"

check-system-requirements:
    @just _check-tool "cargo" "cargo"
    @# just _check-tool "cargo fmt -- --force" "cargo fmt"
    @just _check-tool "cargo-zigbuild" "cargo-zigbuild"
