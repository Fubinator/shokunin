use std::collections::HashMap;
use std::fs;

/// Render a template with the given context.
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
    // println!("output: {}", output);
    if output.contains("{{") {
        Err(format!(
            "Failed to render template, unresolved template tags: {}",
            output
        ))
    } else {
        Ok(output)
    }
}
/// Renders an HTML page with the given title, description, keywords,
/// meta tags, CSS file, content, and copyright notice.
///
/// This function takes in several arguments that correspond to various
/// parts of an HTML page. These arguments are used to construct a
/// context `HashMap` that is passed to the `render_template` function
/// along with a template HTML file. The resulting string returned by
/// `render_template` is the final HTML page that is generated.
///
/// # Arguments
///
/// * `title` - The title of the HTML page.
/// * `description` - The description of the HTML page.
/// * `keywords` - The keywords associated with the HTML page.
/// * `meta` - The meta tags for the HTML page.
/// * `css` - The path to the CSS file used by the HTML page.
/// * `content` - The content of the HTML page.
/// * `copyright` - The copyright notice for the HTML page.
///
/// # Returns
///
/// If the function succeeds, it returns `Ok(html)`, where `html` is the
/// HTML page generated by the function. If the function encounters an
/// error, it returns `Err(error)`, where `error` is a string describing
/// the error that occurred.
///
pub fn render_page(
    title: &str,
    description: &str,
    keywords: &str,
    meta: &str,
    css: &str,
    content: &str,
    copyright: &str,
) -> Result<String, String> {
    let mut context = HashMap::new();
    context.insert("title", title);
    context.insert("description", description);
    context.insert("keywords", keywords);
    context.insert("meta", meta);
    context.insert("css", css);
    context.insert("content", content);
    context.insert("copyright", copyright);

    let template = fs::read_to_string("./template/template.html")
        .map_err(|e| format!("{}", e))?;

    render_template(&template, &context)
}
