use super::AppId;
use super::Component;
use anyhow::Result;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::Deserialize;
#[cfg(feature = "gzip")]
use std::fs::File;
#[cfg(feature = "gzip")]
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection {
    pub version: String,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
    // TODO: architecture
}

impl Collection {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;

        let collection: Collection = from_str(&xml)?;
        Ok(collection)
    }

    #[cfg(feature = "gzip")]
    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let collection: Collection = from_str(&xml)?;
        Ok(collection)
    }

    pub fn find_by_id(&self, id: AppId) -> Vec<&Component> {
        self.components
            .iter()
            .filter(|c| c.id.0 == id.0)
            .collect::<Vec<&Component>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{Category, ComponentKind, Icon, Image, ProjectUrl, Provide};
    use crate::TranslatableVec;
    use crate::{
        AppId, CollectionBuilder, ComponentBuilder, ReleaseBuilder, ScreenshotBuilder,
        TranslatableString,
    };
    use std::convert::TryFrom;
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn spec_example_collection() {
        let c1 = Collection::from_path("./tests/collections/spec_example.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.10")
        .component(
            ComponentBuilder::new(
                AppId::try_from("org.mozilla.Firefox").unwrap(),
                TranslatableString::with_default("Firefox").and_locale("en_GB", "Firefoux")
            )
            .kind(ComponentKind::DesktopApplication)
            .pkgname("firefox-bin")
            .project_license("MPL-2.0".into())
            .keywords(TranslatableVec::with_default(vec!["internet","web", "browser"]).and_locale("fr_FR", vec!["navigateur"]))
            .summary(TranslatableString::with_default("Web browser").and_locale("fr_FR", "Navigateur web"))
            .url(ProjectUrl::Homepage(Url::from_str("https://www.mozilla.com").unwrap()))
            .screenshot(
                ScreenshotBuilder::new().image(Image::Source {
                    url: Url::from_str("https://www.awesomedistro.example.org/en_US/firefox.desktop/main.png").unwrap(),
                    width: Some(800),
                    height: Some(600),
                })
                .image(Image::Thumbnail {
                    url: Url::from_str("https://www.awesomedistro.example.org/en_US/firefox.desktop/main-small.png").unwrap(),
                    width: 200,
                    height: 150,
                }).build()
            )
            .provide(Provide::Binary("firefox".into()))
            .mimetype("text/html")
            .mimetype("text/xml")
            .mimetype("application/xhtml+xml")
            .mimetype("application/vnd.mozilla.xul+xml")
            .mimetype("text/mml")
            .mimetype("application/x-xpinstall")
            .mimetype("x-scheme-handler/http")
            .mimetype("x-scheme-handler/https")
            .category(Category::Unknown("network".into()))
            .category(Category::Unknown("webbrowser".into()))
            .icon(Icon::Stock("web-browser".into()))
            .icon(Icon::Cached("firefox.png".into()))
            .build()
        )
        .component(
            ComponentBuilder::new(
                AppId::try_from("org.freedesktop.PulseAudio").unwrap(),
                TranslatableString::with_default("PulseAudio")
            )
            .summary(TranslatableString::with_default("The PulseAudio sound server"))
            .project_license("GPL-2.0+".into())
            .url(ProjectUrl::Homepage(Url::from_str("https://www.freedesktop.org/wiki/Software/PulseAudio/").unwrap()))
            .provide(Provide::Library("libpulse-simple.so.0".into()))
            .provide(Provide::Library("libpulse.so.0".into()))
            .provide(Provide::Binary("start-pulseaudio-kde".into()))
            .provide(Provide::Binary("start-pulseaudio-x11".into()))
            .release(ReleaseBuilder::new("2.0").build())
            .build()
        )
        .component(
            ComponentBuilder::new(
                AppId::try_from("org.linuxlibertine.LinuxLibertine").unwrap(),
                TranslatableString::with_default("Linux Libertine")
            )
            .kind(ComponentKind::Font)
            .summary(TranslatableString::with_default("Linux Libertine Open fonts"))
            .provide(Provide::Font("LinLibertine_M.otf".into()))
            .build()
        )
        .build();
        assert_eq!(c1, c2);
    }
    #[test]
    fn generic_collection() {
        let c1 =
            Collection::from_path("./tests/collections/fedora-other-repos.xml".into()).unwrap();

        let c2 = CollectionBuilder::new("0.8")
            .component(
                ComponentBuilder::new(
                    AppId::try_from("adobe-release-x86_64").unwrap(),
                    TranslatableString::with_default("Adobe"),
                )
                .pkgname("adobe-release-x86_64")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "Adobe Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    AppId::try_from("livna-release").unwrap(),
                    TranslatableString::with_default("Livna"),
                )
                .pkgname("livna-release")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "Livna Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    AppId::try_from("rpmfusion-free-release").unwrap(),
                    TranslatableString::with_default("RPM Fusion Free"),
                )
                .pkgname("rpmfusion-free-release")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "RPM Fusion Repository Configuration",
                ))
                .build(),
            )
            .component(
                ComponentBuilder::new(
                    AppId::try_from("rpmfusion-nonfree-release").unwrap(),
                    TranslatableString::with_default("RPM Fusion Non-Free"),
                )
                .pkgname("rpmfusion-nonfree-release")
                .metadata_license("CC0-1.0".into())
                .summary(TranslatableString::with_default(
                    "RPM Fusion Repository Configuration",
                ))
                .build(),
            )
            .build();

        assert_eq!(c1, c2);
    }
    #[test]
    fn web_collection() {
        let c = Collection::from_path("./tests/collections/fedora-web-apps.xml".into()).unwrap();

        assert_eq!(c.version, "0.8");
        let comp = c.components.get(0).unwrap();
        assert_eq!(comp.kind, ComponentKind::WebApplication);
        assert_eq!(comp.icons, vec![
            Icon::Remote{
                url: Url::from_str("http://g-ecx.images-amazon.com/images/G/01/kindle/www/ariel/kindle-icon-kcp120._SL90_.png").unwrap(),
                width: None,
                height: None
            }
        ]);
        assert_eq!(
            comp.categories,
            vec![Category::Education, Category::Literature,]
        );

        let keywords = vec!["book", "ebook", "reader"];

        assert_eq!(comp.keywords, Some(TranslatableVec::with_default(keywords)))
    }
}