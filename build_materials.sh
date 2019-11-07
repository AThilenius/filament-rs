#!/bin/sh
set -e

RED='\033[0;31m'
NC='\033[0m'

if [[ -z "$2" ]] ; then
    echo -e "${RED}Invalid arguments.${NC}"
    echo -e "Usage: $0 <source-path> <output-path>"
    exit 1
fi

BASEDIR=$(dirname "$0")
TARGETDIR="${BASEDIR}/target"

SRC_DIR="${1%%/}"
OUT_DIR="${2%%/}"

mkdir -p "${OUT_DIR}"

if [ ! -d "${TARGETDIR}" ]; then
  echo -e "${RED}No target directory!${NC}"
  echo -e "Hint: Did you forget to run cargo build beforehand?"
  exit 1
fi

MATC_PATHS=$(find "${TARGETDIR}" -name "matc.exe" -print)
if [ -z "$MATC_PATHS" ]; then
  echo -e "${RED}Failed to find a matc binary!${NC}"
  echo -e "Hint: Did you forget to run cargo build beforehand?"
  exit 1
fi

MATC_PATH=$(echo "${MATC_PATHS}" | head -1)
echo "Using matc at: ${MATC_PATH}"

# Clear the out bin dir
rm -f "${OUT_DIR}"/*.filamat

find "${SRC_DIR}" -name "*.mat"|while read FNAME; do
  NAME=$(basename "${FNAME}" ".php")
  OUT_FILE="${OUT_DIR}/${NAME%.*}.filamat"
  printf "Compiling: %s \t->\t %s\n" ${FNAME} ${OUT_FILE}
  $MATC_PATH -o $OUT_FILE ${FNAME}
done

echo "Done."
