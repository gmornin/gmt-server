use futures_util::StreamExt;
use goodmorning_services::bindings::services::v1::*;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use std::error::Error;

use goodmorning_services::traits::CollectionItem;

use crate::{components::TexPublishProp, functions::get_tex_userpublishes};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TexPublish {
    #[serde(rename = "_id")]
    pub id: i64,
    pub published: u64,
    pub updated: u64,
    pub title: String,
    pub desc: String,
    pub ext: String,
}

impl TexPublish {
    pub fn as_prop(self, userid: i64) -> TexPublishProp {
        TexPublishProp { base: self, userid }
    }
}

impl CollectionItem<i64> for TexPublish {
    fn id(&self) -> i64 {
        self.id
    }
}

impl TexPublish {
    pub async fn new(
        userid: i64,
        publishid: &mut i64,
        title: String,
        desc: String,
        ext: String,
    ) -> Result<Self, Box<dyn Error>> {
        let now = chrono::Utc::now().timestamp() as u64;
        let s = Self {
            id: *publishid,
            published: now,
            updated: now,
            title,
            desc,
            ext,
        };

        s.save_create(&get_tex_userpublishes(userid)).await?;

        *publishid += 1;

        Ok(s)
    }

    // (items, continued)
    pub async fn list(
        userid: i64,
        mut page: u64,
        page_size: u64,
    ) -> Result<(Vec<TexPublish>, bool), Box<dyn Error>> {
        let collection = get_tex_userpublishes(userid);

        page = page.saturating_sub(1);

        let mut find_options = FindOptions::default();
        find_options.skip = Some(page * page_size); // Skip the first 9 documents
        find_options.limit = Some(page_size as i64 + 1); // Retrieve 11 documents (10th to 20th)
        find_options.batch_size = Some(page_size as u32);
        find_options.sort = Some(doc! {"_id": -1});

        let mut cursor = collection.find(None, find_options).await.unwrap();

        let mut items = Vec::with_capacity(page_size as usize);

        while let Some(document) = cursor.next().await {
            items.push(document?);
            if items.len() == page_size as usize {
                break;
            }
        }

        Ok((items, cursor.next().await.is_some()))
    }

    pub async fn list_prop(
        userid: i64,
        mut page: u64,
        page_size: u64,
    ) -> Result<(Vec<TexPublishProp>, bool), Box<dyn Error>> {
        let collection = get_tex_userpublishes(userid);

        page = page.saturating_sub(1);

        let mut find_options = FindOptions::default();
        find_options.skip = Some(page * page_size); // Skip the first 9 documents
        find_options.limit = Some(page_size as i64 + 1); // Retrieve 11 documents (10th to 20th)
        find_options.batch_size = Some(page_size as u32);
        find_options.sort = Some(doc! {"_id": -1});

        let mut cursor = collection.find(None, find_options).await.unwrap();

        let mut items = Vec::with_capacity(page_size as usize);

        while let Some(document) = cursor.next().await {
            items.push(TexPublishProp {
                base: document?,
                userid,
            });
            if items.len() == page_size as usize {
                break;
            }
        }

        Ok((items, cursor.next().await.is_some()))
    }

    pub async fn total(userid: i64) -> Result<u64, Box<dyn Error>> {
        Ok(get_tex_userpublishes(userid)
            .estimated_document_count(None)
            .await?)
    }
}

impl From<TexPublish> for V1TexUserPublish {
    fn from(val: TexPublish) -> Self {
        V1TexUserPublish {
            id: val.id,
            published: val.published,
            title: val.title,
            desc: val.desc,
            ext: val.ext,
        }
    }
}

impl From<TexPublish> for V1SingleTexUserPublish {
    fn from(val: TexPublish) -> Self {
        V1SingleTexUserPublish {
            id: val.id,
            published: val.published,
            title: val.title,
            desc: val.desc,
            ext: val.ext,
        }
    }
}
