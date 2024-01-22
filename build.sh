#!/bin/bash -eux

OpenSCAD="/Applications/OpenSCAD.app/Contents/MacOS/OpenSCAD"

subcommand="$1"
shift
if [ $# -eq 0 ]; then
	echo "need a file name (*.pcad)"
	exit 1
fi

case $subcommand in
	preview)
		for SOURCE in "$@"; do
			DEST=${SOURCE%.pcad}.png
			echo $DEST $SOURCE
			if [ -e "$DEST" ] && [ "$DEST" -nt "$SOURCE" ]; then
				echo "skip $DEST"
				continue
			fi
			${OpenSCAD} --hardwarning --preview -D "\$burr_scale=10" -D "\$auto_layout=false" -D "\$unit_beveled=true" -o $DEST $SOURCE && :
			if [ $? -ne 0 ]; then
				echo "failed to preview $SOURCE"
			fi
		done 
		;;
	build)
		SOURCE=$1
		SCALE=${2:-10}
		BEVEL=${3:-true}
		AUTO_LAYOUT=${4:-true}
		DEST=${SOURCE%.pcad}_${SCALE}_${BEVEL}.stl
		${OpenSCAD} --hardwarning -D "\$burr_scale=${SCALE}" -D "\$auto_layout=${AUTO_LAYOUT}" -D "\$unit_beveled=${BEVEL}" -o $DEST $SOURCE
		;;
	*)
		echo "preview {}.stl or build {}.stl"
		exit 1
		;;
esac