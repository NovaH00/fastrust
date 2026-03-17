use super::builder::APIApp;

impl<S> APIApp<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Generates the Swagger UI HTML page.
    ///
    /// This method creates an HTML page that embeds Swagger UI, configured
    /// to load the OpenAPI specification from the given path. The Swagger
    /// UI provides an interactive interface for exploring and testing API
    /// endpoints.
    ///
    /// # Arguments
    ///
    /// * `openapi_path` - The path to the OpenAPI JSON specification
    ///
    /// # Returns
    ///
    /// A complete HTML document as a string that includes Swagger UI
    /// via CDN and is configured to load the OpenAPI spec from the
    /// specified path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let html = APIApp::<()>::swagger_html("/openapi.json");
    /// assert!(html.contains("Swagger UI"));
    /// ```
    pub fn swagger_html(openapi_path: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
    <style>
        body {{ margin: 0; padding: 0; }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {{
            SwaggerUIBundle({{
                url: "{openapi_path}",
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "BaseLayout",
                deepLinking: true
            }});
        }};
    </script>
</body>
</html>"#
        )
    }
}
