/*
 * render/html/element/bibliography.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2022 Wikijump Team
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

use super::prelude::*;

pub fn render_bibcite(ctx: &mut HtmlContext, label: &str, brackets: bool) {
    info!("Rendering bibliography citation (label {label}, brackets {brackets})");

    todo!()
}

pub fn render_bibliography(ctx: &mut HtmlContext, title: Option<&str>) {
    info!("Rendering bibliography block (title {})", title.unwrap_or("<default>"));

    todo!()
}
