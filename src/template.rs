// Copyright © 2023 Shokunin (職人). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use reqwest;
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::Path,
};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
/// ## Struct: `PageOptions` - Options for rendering a page template
///
/// This struct contains the options for rendering a page template.
/// These options are used to construct a context `HashMap` that is
/// passed to the `render_template` function.
///
/// # Arguments
///
/// * `author`      - A string representing the author.
/// * `banner`      - A string representing the banner image.
/// * `charset`     - A string representing the character set.
/// * `content`     - A string representing the content.
/// * `copyright`   - A string representing the copyright notice.
/// * `css`         - A string representing the css.
/// * `date`        - A string representing the date.
/// * `description` - A string representing the description.
/// * `generator`   - A string representing the generator.
/// * `image`       - A string representing the image.
/// * `keywords`    - A string representing the keywords.
/// * `lang`        - A string representing the language.
/// * `layout`      - A string representing the layout.
/// * `meta`        - A string representing the meta tags.
/// * `msvalidate_01` - A string representing the Bing site verification.
/// * `name`        - A string representing the site name.
/// * `navigation`  - A string representing the navigation.
/// * `og_description` - A string representing the OpenGraph description.
/// * `og_image_alt`   - A string representing the OpenGraph image alt text.
/// * `og_image`       - A string representing the OpenGraph image.
/// * `og_locale`      - A string representing the OpenGraph locale.
/// * `og_site_name`   - A string representing the OpenGraph site name.
/// * `og_title`       - A string representing the OpenGraph title.
/// * `og_type`        - A string representing the OpenGraph type.
/// * `og_url`         - A string representing the OpenGraph URL.
/// * `subtitle`      - A string representing the subtitle.
/// * `title`        - A string representing the title.
/// * `twitter_card` - A string representing the Twitter card type.
/// * `twitter_creator` - A string representing the Twitter creator.
/// * `twitter_description` - A string representing the Twitter description.
/// * `twitter_image_alt` - A string representing the Twitter image alt text.
/// * `twitter_image` - A string representing the Twitter image.
/// * `twitter_site`  - A string representing the Twitter site.
/// * `twitter_title` - A string representing the Twitter title.
/// * `twitter_url` - A string representing the Twitter URL.
///
pub struct PageOptions<'a> {
    /// A string representing the author of the page.
    pub author: &'a str,
    /// A string representing the banner of the page.
    pub banner: &'a str,
    /// A string representing the charset of the page.
    pub charset: &'a str,
    /// A string representing the content of the page.
    pub content: &'a str,
    /// A string representing the copyright notice of the page.
    pub copyright: &'a str,
    /// A string representing the CSS file of the page.
    pub css: &'a str,
    /// A string representing the date of the page.
    pub date: &'a str,
    /// A string representing the description of the page.
    pub description: &'a str,
    /// A string representing the generator of the page.
    pub generator: &'a str,
    /// A string representing the image of the page.
    pub image: &'a str,
    /// A string representing the google site verification of the page.
    pub google_site_verification: &'a str,
    /// A string representing the keywords of the page.
    pub keywords: &'a str,
    /// A string representing the language of the page.
    pub lang: &'a str,
    /// A string representing the layout of the page.
    pub layout: &'a str,
    /// A string representing the meta tags of the page.
    pub meta: &'a str,
    /// A string representing the bing_site_verification of the page.
    pub bing_site_verification: &'a str,
    /// A string representing the navigation of the page.
    pub name: &'a str,
    /// A string representing the open graph description of the page.
    pub navigation: &'a str,
    /// A string representing the open graph image alt of the page.
    pub og_description: &'a str,
    /// A string representing the open graph image of the page.
    pub og_image_alt: &'a str,
    /// A string representing the open graph locale of the page.
    pub og_image: &'a str,
    /// A string representing the open graph site name of the page.
    pub og_locale: &'a str,
    /// A string representing the open graph title of the page.
    pub og_site_name: &'a str,
    /// A string representing the open graph type of the page.
    pub og_title: &'a str,
    /// A string representing the open graph url of the page.
    pub og_type: &'a str,
    /// A string representing the site name of the page.
    pub og_url: &'a str,
    /// A string representing the subtitle of the page.
    pub subtitle: &'a str,
    /// A string representing the title of the page.
    pub title: &'a str,
    /// A string representing the twitter card of the page.
    pub twitter_card: &'a str,
    /// A string representing the twitter creator of the page.
    pub twitter_creator: &'a str,
    /// A string representing the twitter description of the page.
    pub twitter_description: &'a str,
    /// A string representing the twitter image alt of the page.
    pub twitter_image_alt: &'a str,
    /// A string representing the twitter image of the page.
    pub twitter_image: &'a str,
    /// A string representing the twitter site of the page.
    pub twitter_site: &'a str,
    /// A string representing the twitter title of the page.
    pub twitter_title: &'a str,
    /// A string representing the twitter url of the page.
    pub twitter_url: &'a str,
}

/// ## Function: `render_template` - Render a template with the given context
///
/// This function takes in a template string and a context hash map as
/// arguments. The template string is a string containing placeholders
/// for values that will be replaced with the values in the context hash
/// map. The placeholders take the form "{{key}}", where "key" is the
/// key of a value in the context hash map. For example, a template
/// string might contain the placeholder "{{name}}" which would be
/// replaced with the value of the "name" key in the context hash map.
///
/// The function replaces all placeholders in the template string with
/// their corresponding values from the context hash map. If a
/// placeholder cannot be replaced (i.e. there is no corresponding key
/// in the context hash map), an error is returned with a message
/// indicating which placeholder could not be resolved.
///
/// If all placeholders are successfully replaced, the resulting string
/// with replaced placeholders is returned as a `String` wrapped in a
/// `Result::Ok`. If an error occurs, an error message is returned as a
/// `String` wrapped in a `Result::Err`.
///
pub fn render_template(
    template: &str,
    context: &HashMap<&str, &str>,
) -> Result<String, String> {
    let mut output = template.to_owned();
    for (key, value) in context {
        output = output.replace(&format!("{{{{{}}}}}", key), value);
    }
    // Check if all keys have been replaced
    if output.contains("{{") {
        Err(format!(
            "Failed to render template, unresolved template tags: {}",
            output
        ))
    } else {
        Ok(output)
    }
}

/// ## Function: `render_page` - Render an HTML page
///
/// Renders an HTML page with the given title, date, description,
/// keywords, meta tags, CSS file, content, and copyright notice.
///
/// This function takes in several arguments that correspond to various
/// parts of an HTML page. These arguments are used to construct a
/// context `HashMap` that is passed to the `render_template` function
/// along with a template HTML file. The resulting string returned by
/// `render_template` is the final HTML page that is generated.
///
/// # Arguments
///
/// * `author`              - A string representing the author.
/// * `banner`              - A string representing the banner image.
/// * `charset`             - A string representing the character set.
/// * `content`             - A string representing the content.
/// * `copyright`           - A string representing the copyright notice.
/// * `css`                 - A string representing the css.
/// * `date`                - A string representing the date.
/// * `description`         - A string representing the description.
/// * `generator`           - A string representing the generator.
/// * `google_site_verification` - A string representing the Google site verification.
/// * `image`               - A string representing the image.
/// * `keywords`            - A string representing the keywords.
/// * `lang`                - A string representing the language.
/// * `layout`              - A string representing the layout.
/// * `meta`                - A string representing the meta tags.
/// * `bing_site_verification`       - A string representing the bing_site_verification.
/// * `name`                - A string representing the site name.
/// * `navigation`          - A string representing the navigation.
/// * `og_description`      - A string representing the OpenGraph description.
/// * `og_image_alt`        - A string representing the OpenGraph image alt text.
/// * `og_image`            - A string representing the OpenGraph image.
/// * `og_locale`           - A string representing the OpenGraph locale.
/// * `og_site_name`        - A string representing the OpenGraph site name.
/// * `og_title`            - A string representing the OpenGraph title.
/// * `og_type`             - A string representing the OpenGraph type.
/// * `og_url`              - A string representing the OpenGraph URL.
/// * `subtitle`            - A string representing the subtitle.
/// * `title`               - A string representing the title.
/// * `twitter_card`        - A string representing the Twitter card type.
/// * `twitter_creator`     - A string representing the Twitter creator.
/// * `twitter_description` - A string representing the Twitter description.
/// * `twitter_image_alt`   - A string representing the Twitter image alt text.
/// * `twitter_image`       - A string representing the Twitter image.
/// * `twitter_site`        - A string representing the Twitter site.
/// * `twitter_title`       - A string representing the Twitter title.
/// * `twitter_url`         - A string representing the Twitter URL.
///
/// # Returns
///
/// If the function succeeds, it returns `Ok(html)`, where `html` is the
/// HTML page generated by the function. If the function encounters an
/// error, it returns `Err(error)`, where `error` is a string describing
/// the error that occurred.
///
pub fn render_page(
    options: &PageOptions,
    template_path: &String,
    layout: &String,
) -> Result<String, String> {
    let mut context = HashMap::new();
    context.insert("author", options.author);
    context.insert("banner", options.banner);
    context.insert("charset", options.charset);
    context.insert("content", options.content);
    context.insert("copyright", options.copyright);
    context.insert("css", options.css);
    context.insert("date", options.date);
    context.insert("description", options.description);
    context.insert("generator", options.generator);
    context.insert(
        "google_site_verification",
        options.google_site_verification,
    );
    context.insert("image", options.image);
    context.insert("keywords", options.keywords);
    context.insert("lang", options.lang);
    context.insert("meta", options.meta);
    context.insert(
        "bing_site_verification",
        options.bing_site_verification,
    );
    context.insert("name", options.name);
    context.insert("navigation", options.navigation);
    context.insert("og_description", options.og_description);
    context.insert("og_image_alt", options.og_image_alt);
    context.insert("og_image", options.og_image);
    context.insert("og_locale", options.og_locale);
    context.insert("og_site_name", options.og_site_name);
    context.insert("og_title", options.og_title);
    context.insert("og_type", options.og_type);
    context.insert("og_url", options.og_url);
    context.insert("subtitle", options.subtitle);
    context.insert("title", options.title);
    context.insert("twitter_card", options.twitter_card);
    context.insert("twitter_creator", options.twitter_creator);
    context.insert("twitter_description", options.twitter_description);
    context.insert("twitter_image_alt", options.twitter_image_alt);
    context.insert("twitter_image", options.twitter_image);
    context.insert("twitter_site", options.twitter_site);
    context.insert("twitter_title", options.twitter_title);
    context.insert("twitter_url", options.twitter_url);
    if layout == "index" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("index.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "post" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("post.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "page" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("page.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "tag" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("tag.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "category" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("category.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "archive" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("archive.html"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "rss" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("rss.xml"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "sitemap" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("sitemap.xml"),
            )
            .unwrap(),
            &context,
        )
    } else if layout == "atom" {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("atom.xml"),
            )
            .unwrap(),
            &context,
        )
    } else {
        render_template(
            &fs::read_to_string(
                Path::new(template_path).join("template.html"),
            )
            .unwrap(),
            &context,
        )
    }
}

/// Custom error type to handle both reqwest and io errors
#[derive(Debug)]
pub enum TemplateError {
    /// Error from reqwest
    Reqwest(reqwest::Error),
    /// Error from io
    Io(std::io::Error),
}

impl From<reqwest::Error> for TemplateError {
    fn from(err: reqwest::Error) -> TemplateError {
        TemplateError::Reqwest(err)
    }
}

impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> TemplateError {
        TemplateError::Io(err)
    }
}

/// Creates a directory if it does not exist. If the directory already
/// exists, the function will return `Ok(())`.
///
/// # Arguments
///
/// - `path` - The path to the directory.
///
pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir(path).or_else(|e| match e.kind() {
        io::ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}

/**
 * Creates a template folder based on the provided template path or uses
 * the default template folder
 *
 * - If a URL is provided as the template path, the function downloads
 *   the template files to a temporary directory.
 * - If a local path is provided, the function uses it as the template
 *   directory path.
 * - If no path is provided, the function downloads the default template
 *   files to a temporary directory.
 *
 * # Arguments
 *
 * * `template_path` - An optional `&str` containing the path to the
 *   template folder
 *
 * # Returns
 *
 * Returns a `Result` that contains `()` if successful, or a
 * `TemplateError` if an error occurs.
 *
 */
pub fn create_template_folder(
    template_path: Option<&String>,
) -> Result<String, TemplateError> {
    // Get the current working directory
    let current_dir = std::env::current_dir()?;

    // Determine the template directory path based on the provided argument or use the default path
    let template_dir_path = match template_path {
        Some(path) => {
            if path.starts_with("http://")
                || path.starts_with("https://")
            {
                // If a URL is provided, download the template files to a temporary directory
                let tempdir = tempfile::tempdir()?;
                let template_dir_path = tempdir.into_path();
                println!(
                    "Creating temporary directory for template: {:?}",
                    template_dir_path
                );

                let url = path;
                let files = [
                    "index.html",
                    "template.html",
                    "template.json",
                    "post.html",
                    "page.html",
                    "tag.html",
                    "category.html",
                    "archive.html",
                    "rss.xml",
                    "sitemap.xml",
                    "atom.xml",
                ];

                for file in files.iter() {
                    let file_url = format!("{}/{}", url, file);
                    let file_path = template_dir_path.join(file);
                    let mut download =
                        reqwest::blocking::get(&file_url)?;
                    let mut file = File::create(&file_path)?;
                    download.copy_to(&mut file)?;
                    println!(
                        "Downloaded template file to: {:?}",
                        file_path
                    );
                }

                template_dir_path
            } else {
                // If a local path is provided, use it as the template
                // directory path
                // println!("Using local template directory: {}", path);
                current_dir.join(path)
            }
        }
        None => {
            // If no path is provided, download the default template files to a temporary directory
            let tempdir = tempfile::tempdir()?;
            let template_dir_path = tempdir.into_path();
            println!(
                "Creating temporary directory for default template: {:?}",
                template_dir_path
            );

            let url = "https://raw.githubusercontent.com/sebastienrousseau/shokunin/main/template/";
            let files = [
                "index.html",
                "template.html",
                "template.json",
                "post.html",
                "page.html",
                "tag.html",
                "category.html",
                "archive.html",
                "rss.xml",
                "sitemap.xml",
                "atom.xml",
            ];

            for file in files.iter() {
                let file_url = format!("{}/{}", url, file);
                let file_path = template_dir_path.join(file);
                let mut download = reqwest::blocking::get(&file_url)?;
                let mut file = File::create(&file_path)?;
                download.copy_to(&mut file)?;
                println!(
                    "Downloaded default template file to: {:?}",
                    file_path
                );
            }

            template_dir_path
        }
    };
    Ok(String::from(template_dir_path.to_str().unwrap()))
}
