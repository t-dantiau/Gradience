> [!IMPORTANT]
> This is a new work in progress version of Gradience which aims to be a fully featured replacement but which isn't supported or approved by any member of the former Gradience team. I hope that the Gradience community will use this new version of Gradience (φοῖνιξ Edition). I kept the name Gradience because I liked it, but I'm open to any suggestions about the design side of this upcoming theming app
>
>
<h1 align="center">
  <img src="https://github.com/GradienceTeam/Gradience/raw/main/data/icons/hicolor/scalable/apps/com.github.GradienceTeam.Gradience.svg" alt="Gradience" width="192" height="192"/>
  <br>
  Gradience φοῖνιξ
</h1>

<p align="center">
  <strong>Change the look of GNOME, with ease</strong>
</p>

## CLI

The CLI version of Gradience is available as a standalone binary. You can download it from [GitHub Releases](https://github.com/t-dantiau/Gradience/releases) or build it yourself from sources.

Documentation for the CLI is available as markdown [here](./CommandLineHelp.md) or by using the `--help` flag.

## Library

Gradience Library can be used by any rust program for doing GNOME theming. There is one cargo feature that can be enabled if you want to be able to download presets from the online preset store (`online`), this feature is disabled by default.

If you only use the library, you need to provide shell templates yourself, you can look at gradience cli's code to see how it has been done for packaging everything in one binary.

## License

Gradience φοῖνιξ is licensed under the [EUPL](./LICENSE)

## About

This library is currently work in progress, but I choosed to keep the name, if anyone has a better name, I'll be open to change!