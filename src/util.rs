use crate::gitlab;
use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;

pub fn handle_response_status<T>(status: StatusCode, resource: String, response: Response) -> T
where
    T: DeserializeOwned,
{
    match status {
        StatusCode::OK => parse_response::<T>(response, resource),
        StatusCode::CREATED => parse_response::<T>(response, resource),
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized, please make sure your personal access token is correct!")
        }
        StatusCode::NOT_FOUND => {
            let json_response = response.json::<gitlab::GitlabError>().expect(
                format!(
                    "An unkown error happened while creating your new {}!",
                    resource
                )
                .as_str(),
            );
            panic!("Not Found error: {}", json_response.message)
        }
        StatusCode::BAD_REQUEST => {
            let text = response.text().unwrap();
            println!("text: {}", text);
            // let json_response = response.json().expect(
            //     format!(
            //         "An unkown error happened while creating your new {}!",
            //         resource
            //     )
            //     .as_str(),
            // );
            panic!(
                "A validation error ocurred while creating your new {}",
                resource
            );
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("Internal server error, please contact Gitlab if you see this");
        }
        _ => panic!(
            "An unexpected error ocurred while creating your {}",
            resource
        ),
    }
}

fn parse_response<T>(response: Response, resource: String) -> T
where
    T: DeserializeOwned,
{
    let new_entity = response.json::<T>().expect(
        format!(
            "An error ocurred while reading the response to create a {}",
            resource
        )
        .as_str(),
    );
    return new_entity;
}
