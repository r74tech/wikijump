/*
 * utils/slug.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
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

use crate::utils::replace_in_place;
use wikidot_normalize::normalize;

/// Normalize a name to a slug. Does not preseve `:`.
///
/// Meant for use in sites and users.
pub fn get_regular_slug<S: Into<String>>(name: S) -> String {
    let mut slug = name.into();
    replace_in_place(&mut slug, ":", "-");
    normalize(&mut slug);
    slug
}