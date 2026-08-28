use crate::config::config::Config;

//so now we have the hugo config which is written first and the loc variable is appened to the
//create_md_from_file's string
//also this function will just be called once and all teh values will be stored and written to
//each md files
pub(crate) fn get_hugo_config(config: &Config) -> (String, Option<String>, Option<String>, String) {
    let loc = config.hugo_config.loc.clone();
    let owner = config.hugo_config.owner.clone();
    let date = config.hugo_config.date.clone();
    let draft = config.hugo_config.draft.clone();

    (loc, owner, date, draft)
}
