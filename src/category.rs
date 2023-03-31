// SPDX-License-Identifier: BSD-2-Clause-Patent

use strum_macros::{Display, EnumIter, EnumString};

#[derive(EnumIter, EnumString, Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Category {
    #[strum(serialize = "Select Category")]
    None,
    #[strum(serialize = "Amateur")]
    Amateur,
    #[strum(serialize = "Anal")]
    Anal,
    #[strum(serialize = "Anime Games")]
    AnimeGames,
    #[strum(serialize = "Asian")]
    Asian,
    #[strum(serialize = "Bareback")]
    Bareback,
    #[strum(serialize = "BDSM")]
    Bdsm,
    #[strum(serialize = "Bears")]
    Bears,
    #[strum(serialize = "Bisexual")]
    Bisexual,
    #[strum(serialize = "Black")]
    Black,
    #[strum(serialize = "Books & Magazines")]
    BooksMagazines,
    #[strum(serialize = "Chubbies")]
    Chubbies,
    #[strum(serialize = "Clips")]
    Clips,
    #[strum(serialize = "Comic & Yaoi")]
    ComicYaoi,
    #[strum(serialize = "Daddies / Sons")]
    DaddiesSons,
    #[strum(serialize = "Dildos")]
    Dildos,
    #[strum(serialize = "Fan Sites")]
    FanSites,
    #[strum(serialize = "Fetish")]
    Fetish,
    #[strum(serialize = "Fisting")]
    Fisting,
    #[strum(serialize = "Grey / Older")]
    GreyOlder,
    #[strum(serialize = "Group-Sex")]
    GroupSex,
    #[strum(serialize = "Homemade")]
    Homemade,
    #[strum(serialize = "Hunks")]
    Hunks,
    #[strum(serialize = "Images")]
    Images,
    #[strum(serialize = "Interracial")]
    Interracial,
    #[strum(serialize = "Jocks")]
    Jocks,
    #[strum(serialize = "Latino")]
    Latino,
    #[strum(serialize = "Mature")]
    Mature,
    #[strum(serialize = "Media Programs")]
    MediaPrograms,
    #[strum(serialize = "Member")]
    Member,
    #[strum(serialize = "Middle Eastern")]
    MiddleEastern,
    #[strum(serialize = "Military")]
    Military,
    #[strum(serialize = "Oral-Sex")]
    OralSex,
    #[strum(serialize = "Softcore")]
    Softcore,
    #[strum(serialize = "Solo")]
    Solo,
    #[strum(serialize = "Straight older")]
    StraightOlder,
    #[strum(serialize = "Straight younger")]
    StraightYounger,
    #[strum(serialize = "Themed Movie")]
    ThemedMovie,
    #[strum(serialize = "Trans")]
    Trans,
    #[strum(serialize = "TV / Episodes")]
    TvEpisodes,
    #[strum(serialize = "Twinks")]
    Twinks,
    #[strum(serialize = "Vintage")]
    Vintage,
    #[strum(serialize = "Voyeur")]
    Voyeur,
    #[strum(serialize = "Wrestling and Sports")]
    WrestlingAndSports,
    #[strum(serialize = "Youngblood")]
    Youngblood
}