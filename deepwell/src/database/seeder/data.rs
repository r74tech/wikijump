/*
 * database/seeder/data.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2025 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::models::sea_orm_active_enums::UserType;
use anyhow::Result;
use ftml::layout::Layout;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use time::Date;

#[derive(Debug)]
pub struct SeedData {
    pub users: Vec<User>,
    pub sites: Vec<Site>,
    pub pages: HashMap<String, Vec<Page>>,
    pub files: HashMap<String, HashMap<String, Vec<File>>>,
    pub filters: Vec<Filter>,
}

impl SeedData {
    pub fn load(directory: &Path) -> Result<Self> {
        let mut path: PathBuf = directory.join("filename");

        // Load user data
        let users: Vec<User> = Self::load_json(&mut path, "users")?;

        // Load site data
        let sites: Vec<Site> = Self::load_json(&mut path, "sites")?;

        // Load page data
        let mut site_pages: HashMap<String, Vec<Page>> =
            Self::load_json(&mut path, "pages")?;

        for (site, pages) in &mut site_pages {
            // Verify that the site exists
            assert!(
                sites.iter().any(|s| &s.slug == site),
                "No site with slug {}",
                site,
            );

            // Fetch wikitext from file
            for page in pages {
                page.wikitext = Self::load_wikitext(&mut path, &page.wikitext_filename)?;
            }
        }

        // Load file data
        let files: HashMap<String, HashMap<String, Vec<File>>> =
            Self::load_json(&mut path, "files")?;

        // Load filter data
        let filters: Vec<Filter> = Self::load_json(&mut path, "filters")?;

        // Build and return
        Ok(SeedData {
            users,
            sites,
            pages: site_pages,
            files,
            filters,
        })
    }

    fn load_json<T>(path: &mut PathBuf, filename: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        path.set_file_name(filename);
        path.set_extension("json");
        debug!("Loading JSON from {}", path.display());

        let mut file = fs::File::open(path)?;
        let data = serde_json::from_reader(&mut file)?;
        Ok(data)
    }

    fn load_wikitext(path: &mut PathBuf, filename: &Path) -> Result<String> {
        path.set_file_name(filename);
        path.set_extension("ftml");
        debug!("Loading wikitext from {}", path.display());

        let wikitext = fs::read_to_string(path)?;
        Ok(wikitext)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub email: String,

    #[serde(rename = "type")]
    pub user_type: UserType,
    pub password: Option<String>,
    pub locales: Vec<String>,
    pub real_name: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<Date>,
    pub location: Option<String>,
    pub biography: Option<String>,
    pub user_page: Option<String>,
    pub aliases: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Site {
    pub slug: String,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default)]
    pub domains: Vec<String>,
    pub name: String,
    pub tagline: String,
    pub description: String,

    #[serde(default)]
    pub default_page: Option<String>,
    pub layout: Option<Layout>,
    pub locale: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Page {
    pub slug: String,
    pub title: String,

    #[serde(default)]
    pub alt_title: Option<String>,

    #[serde(skip)]
    pub wikitext: String,

    #[serde(rename = "wikitext")]
    pub wikitext_filename: PathBuf,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Filter {
    pub regex: String,
    pub description: String,
    pub case_sensitive: bool,

    #[serde(rename = "site")]
    pub site_slug: Option<String>,

    #[serde(default)]
    pub user: bool,

    #[serde(default)]
    pub email: bool,

    #[serde(default)]
    pub page: bool,

    #[serde(default)]
    pub file: bool,

    #[serde(default)]
    pub forum: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct File {
    pub name: String,
    pub path: PathBuf,

    #[serde(default)]
    pub overwrite: Option<PathBuf>,

    #[serde(default)]
    pub deleted: bool,
}
