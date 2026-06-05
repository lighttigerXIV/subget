# About
A small program to download subtitles from OpenSubtitles.

I felt the need to have a quick way to download subtitles do to my Stremio and VLC setup so i created a small terminal program to download subtitles with multiple languages from OpenSubtitles. 

# Screen Shots
<img width="499" height="111" alt="image" src="https://github.com/user-attachments/assets/8e9fe337-81ba-4fae-9721-c94867071bc4" />

<img width="851" height="309" alt="image" src="https://github.com/user-attachments/assets/9ef1ca42-545a-47ea-96fb-0cc2a796473c" />

# Install
To download the program just go to the latest releases and download the version for your OS.

<a href="https://github.com/lighttigerXIV/subget/releases">
<img height="60" alt="github-download" src="https://github.com/user-attachments/assets/b814cff5-b5b0-4ba4-af1f-6372efcc2e01" />
</a>

After downloading you can open the terminal and move the program to where you see fit. I recommend copying it to global location.

On Linux:
```
sudo cp subget /usr/local/bin
```

On windows:
```
cp subget.exe $env:LocalAppData\Microsoft\WindowsApps\
```

# Config
When opening for the first time, the program asks for the API Key and the languages you want to use.
<img width="731" height="162" alt="image" src="https://github.com/user-attachments/assets/b5c8161b-9784-45ac-be36-15f7f7774bbc" />

> [!NOTE]
> You can change the key, languages and download path if you go to the configuration file. `~/.config/subget/config.json`

Example:
```json
{
  "api_key": "QQWIsjl!@IL!@L!JIL@L!I@JL!@JI",
  "languages": [
    "en",
    "pt-PT",
    "pt-BR",
    "es",
    "fr"
  ],
  "download_dir": null
}
```

# How To Use
To use this app just select Show or Movie and it will ask for it's name. In case of a show it will ask for the season and episode number.
