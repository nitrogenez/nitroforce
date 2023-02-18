## **INSTALL VIA [HOMEBREW](https://brew.sh/)**

`brew install android-platform-tools`

## **MANUAL INSTALLATION**

+ Delete old installation: `rm -rf ~/.android-sdk-macosx/*`
+ Go [here](https://developer.android.com/studio/releases/platform-tools.html) and click on `SDK Platform-Tools for Mac`
+ Navigate to Downloads dir: `cd ~/Downloads`
+ Unzip tools: `unzip platform-tools-latest*.zip`
+ Move contents anywhere, e.g:
  - `mv platform-tools-latest.../ ~/.android-sdk-macosx/platform-tools`
+ Add `platform-tools` to `$PATH`:
  - `echo "export PATH=$PATH:~/.android-sdk-macosx/platform-tools/" >> ~/.bash_profile`
+ Source bash profile: `source ~/.bash_profile`

## **EXAMPLE (MANUAL)**
```bash
rm -rf ~/.android-sdk-macosx/*
cd ~/Downloads
unzip platform-tools*
mv platform-tools-latest... ~/.android-sdk-macosx/platform-tools
echo "export PATH=$PATH:~/.android-sdk-macosx/platform-tools/" >> ~/.bash_profile
source ~/.bash_profile
```