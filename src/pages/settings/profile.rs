use std::error::Error;

use actix_files::NamedFile;
use actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse};

use crate::{
    components::{topbar_option_from_req, DetailsProp, ProfileEditBadges},
    functions::from_res,
    structs::TexProfile,
    CREATE_ACC, CSP_BASE,
};

#[get("/profile")]
pub async fn profile(req: HttpRequest) -> HttpResponse {
    from_res(profile_task(&req).await, &req).await
}

async fn profile_task(req: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let (topbar, account) = match topbar_option_from_req(req).await? {
        Ok(Some(stuff)) => stuff,
        Ok(None) => {
            return Ok(NamedFile::open_async(CREATE_ACC.get().unwrap())
                .await?
                .into_response(req))
        }
        Err(res) => return Ok(res),
    };

    let userpf = TexProfile::find_default(account.id).await?.profile;
    let id = account.id;
    // let userpf = read_profile(account.id, goodmorning_services::structs::GMServices::Tex).await?;
    let username = html_escape::encode_safe(&account.username);
    let status = html_escape::encode_safe(&account.status);
    let desc = html_escape::encode_safe(&userpf.description);
    let badges = yew::ServerRenderer::<ProfileEditBadges>::with_props(|| DetailsProp {
        details: userpf.details,
    })
    .render()
    .await;

    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="/static/css/main.css" />
    <link rel="stylesheet" href="/static/css/topbar.css" />
    <link rel="stylesheet" href="/static/css/sprofile.css" />
    <link rel="stylesheet" href="/static/css/settings.css" />
    <link rel="stylesheet" href="/static/css/topbar-signedout.css" />
    <link rel="stylesheet" href="/static/css/topbar-loggedin.css" />
    <link rel="stylesheet" href="/static/css/dark/main.css" />
    <link rel="stylesheet" href="/static/css/dark/sprofile.css" />
    <link rel="stylesheet" href="/static/css/dark/settings.css" />
    <link rel="stylesheet" href="/static/css/dark/topbar.css" />
    <link rel="stylesheet" href="/static/css/dark/topbar-signedout.css" />
    <link
      rel="shortcut icon"
      href="/static/images/favicon-dark.svg"
      type="image/x-icon"
    />
    <title>Public profile - Settings</title>
  </head>
  <body>
    <dialog id="addd">
      <div class="x">&#x2715;</div>
      <h2>Select a detail item</h2>
      <div id="list-container">
        <ul id="details-list">
          <li field="cake day"><img src="/static/icons/cake.svg" />Cake day</li>
          <li field="birthday"><img src="/static/icons/cake.svg" />Birthday</li>
          <li field="location">
            <img src="/static/icons/location.svg" />Location
          </li>
          <li field="occupation">
            <img src="/static/icons/suitcase.svg" />Occupation
          </li>
          <li field="company">
            <img src="/static/icons/business.svg" />Company
          </li>
          <li field="school"><img src="/static/icons/school.svg" />School</li>
          <li field="education">
            <img src="/static/icons/education.svg" />Education level
          </li>
        </ul>
        <ul id="contacts-list">
          <li field="email"><img src="/static/icons/envolope.svg" />Email</li>
          <li field="matrix"><img src="/static/icons/matrix.svg" />Matrix</li>
          <li field="mastodon">
            <img src="/static/icons/mastodon.svg" />Mastodon
          </li>
          <li field="lemmy"><img src="/static/icons/lemmy.svg" />Lemmy</li>
          <li field="github"><img src="/static/icons/github.svg" />Github</li>
          <li field="gitlab"><img src="/static/icons/envolope.svg" />Gitlab</li>
          <li field="bitbucket">
            <img src="/static/icons/bitbucket.svg" />Bitbucket
          </li>
          <li field="reddit"><img src="/static/icons/reddit.svg" />Reddit</li>
          <li field="discord">
            <img src="/static/icons/discord.svg" />Discord
          </li>
          <li field="twitter">
            <img src="/static/icons/twitter.svg" />Twitter
          </li>
          <li field="youtube">
            <img src="/static/icons/youtube.svg" />YouTube
          </li>
          <li field="odysee"><img src="/static/icons/odysee.svg" />Odysee</li>
          <li field="website"><img src="/static/icons/link.svg" />Website (URL)</li>
        </ul>
      </div>
    </dialog>
    {topbar}
    <div id="bottom">
      <div id="pages">
        <div class="selected"><img src="/static/icons/user.svg">Public profile</div>
        <div><img src="/static/icons/settings.svg"><a href="/settings/account">Account</a></div>
      </div>
      <div>
    <div class="container">
      <div id="profile-top">
        <label id="pfp-container">
          <input type="file" accept=".png" />
          <img src="/api/generic/v1/pfp/id/{id}" width="100" height="100" id="pfp" />
          <img src="/static/icons/edit.svg" id="pfp-overlay" />
        </label>
        <div id="profile-top-right">
          <input
            type="text"
            id="username"
            placeholder="Username"
            value="{username}"
            style="width: 200px"
          /><img class="save" src="/static/icons/save.svg" field="username" />
          <br />
          <input
            type="text"
            id="status"
            placeholder="Status"
            value="{status}"
            style="width: 200px"
          /><img class="save" src="/static/icons/save.svg" field="status" />
        </div>
      </div>
      <center id="bio">
        <textarea rows="4" cols="55" id="bio-textarea" placeholder="Personal description here...">{desc}</textarea>
      </center>
      <ul id="badges">
        {badges}
      </ul>
      <div id="details-buts">
        <img id="add" src="/static/icons/plus.svg" />
        <img class="save" src="/static/icons/save.svg" field="profile" />
      </div>
    </div>
    </div>
    </div>
    <div id="backdrop" style="display: none"></div>
    <script src="/static/scripts/settings.js" defer></script>
    <script src="/static/scripts/sprofile.js" defer></script>
  </body>
</html>
            "#
    );

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(("Content-Security-Policy", CSP_BASE.get().unwrap().as_str()))
        .body(html))
}
