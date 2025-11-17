#!/usr/bin/env bash
# -------------------------------------------------------------
# Convertit une skybox en croix (layout 4x3) en strip vertical
# utilisable par Bevy / WGPU.
#
# Format d'entrée (4 colonnes × 3 lignes de carrés):
#
#   Noir   | +Y     | Noir   | Noir
#   -X     | +Z     | +X     | -Z
#   Noir   | -Y     | Noir   | Noir
#
# Commande :
#   ./make_skybox_strip.sh skybox_cross.png
#
# Résultat :
#   skybox_cross_strip.png (6 faces empilées verticalement)
#   Ordre : +X, -X, +Y, -Y, +Z, -Z
# -------------------------------------------------------------

set -e

if [ $# -ne 1 ]; then
  echo "Usage: $0 input_cross.png"
  exit 1
fi

INPUT="$1"
BASENAME=$(basename "$INPUT" .png)

# Étape 1 : découpe en 12 cases (4 colonnes × 3 lignes)
magick "$INPUT" -crop 4x3@ +repage +adjoin "${BASENAME}_face_%d.png"

# Étape 2 : réassemble uniquement les 6 bonnes faces dans l'ordre Bevy
magick \
  "${BASENAME}_face_6.png" \
  "${BASENAME}_face_4.png" \
  "${BASENAME}_face_1.png" \
  "${BASENAME}_face_9.png" \
  "${BASENAME}_face_5.png" \
  "${BASENAME}_face_7.png" \
  -append "${BASENAME}_strip.png"

# Étape 3 : nettoyage (facultatif)
rm "${BASENAME}_face_"*.png

echo "Conversion finished: skybox.png was generated."

mv "${BASENAME}_strip.png" ../assets/skybox.png
