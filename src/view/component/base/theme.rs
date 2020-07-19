
#[derive(Clone)]
pub struct Theme {

}

impl Theme {

    pub fn new(theme:&str) -> String {
        match theme {
            "bootstrap" => Theme::create_theme_bootstrap(),
            "clean" => Theme::create_theme_clean(),
            _ => Theme::create_theme_clean()
        }
    }

    pub fn get_theme_class(theme:&str) -> &str {
        match theme {
            // https://getbootstrap.com/docs/4.5/layout/grid/
            "bootstrap" => "container-fluid",
            "clean" => "container-clean",
            _ => "container-fluid"
        }
    }

    /*
     * Bootstrap v4.5.0 (https://getbootstrap.com/)
     * Copyright 2011-2020 The Bootstrap Authors
     * Copyright 2011-2020 Twitter, Inc.
     * Licensed under MIT (https://github.com/twbs/bootstrap/blob/master/LICENSE)
     */
    fn create_theme_bootstrap() -> String {
        r#"
        
        "#.to_string()
    }

    fn create_theme_clean() -> String {
        r#"
        
        "#.to_string()
    }
}