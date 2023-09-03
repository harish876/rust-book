use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database,
};
use std::{env, io::Error};

use crate::schemas::project_schema::{Owner, Project};

pub struct DBMongo {
    db: Database,
}

impl DBMongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error in loading env Variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("Error in connecting to the DB");

        let db = client.database("rustup");
        Self { db }
    }

    fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    //Owners Logic
    pub async fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let new_doc = Owner {
            _id: None,
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };

        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let data = col
            .insert_one(new_doc, None)
            .await
            .expect("Error Inserting Record into owner");
        let new_owner = Owner {
            _id: data.inserted_id.as_object_id(),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };
        Ok(new_owner)
    }

    pub async fn get_owners(&self) -> Result<Vec<Owner>, Error> {
        let mut owners: Vec<Owner> = Vec::new();
        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let mut cursors = col.find(None, None).await.expect("Failed to fetch owners");
        while let Some(owner) = cursors
            .try_next()
            .await
            .expect("Error Mapping through cluster")
        {
            owners.push(owner)
        }
        Ok(owners)
    }

    pub async fn single_owner(&self, id: &String) -> Result<Owner, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : object_id};
        let col = DBMongo::col_helper::<Owner>(&self, "owner");
        let owner_details = col
            .find_one(filter, None)
            .await
            .expect("Error in getting employee");
        Ok(owner_details.unwrap())
    }

    pub async fn create_project(&self, new_project: Project) -> Result<Project, Error> {
        let new_doc = Project {
            _id: None,
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        let col = DBMongo::col_helper::<Project>(&self, "project");

        let data = col
            .insert_one(new_doc, None)
            .await
            .expect("Error inserting project");

        let new_project = Project {
            _id:data.inserted_id.as_object_id(),
            owner_id: new_project.owner_id.clone(),
            name: new_project.name.clone(),
            description: new_project.description.clone(),
            status: new_project.status.clone(),
        };
        Ok(new_project)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let mut projects: Vec<Project> = Vec::new();
        let col = DBMongo::col_helper::<Project>(&self, "project");
        let mut cursors = col.find(None, None).await.expect("Failed to fetch projects");
        while let Some(project) = cursors
            .try_next()
            .await
            .expect("Error Mapping through cluster")
        {
            projects.push(project)
        }
        Ok(projects)
    }

    pub async fn single_project(&self, id: &String) -> Result<Project, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : object_id};
        let col = DBMongo::col_helper::<Project>(&self, "project");
        let project_details = col
            .find_one(filter, None)
            .await
            .expect("Error in getting employee");
        Ok(project_details.unwrap())
    }

}
