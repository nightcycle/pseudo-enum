source .env/Scripts/Activate
pyinstaller --onefile src/__init__.py -n pseudo-enum --additional-hooks-dir=hooks
# dist/pseudo-enum.exe build
