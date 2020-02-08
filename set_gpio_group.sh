GPIO_DIR=/sys/class/gpio

set -e

echo "Adding group gpio"
addgroup gpio || true
echo "Changing gpio group ownership"
chown -R :gpio $GPIO_DIR
echo "Changing gpio export permissions."
for name in "export" "unexport"; do
    chmod g+w $GPIO_DIR/$name
done
