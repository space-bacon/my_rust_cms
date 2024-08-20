use crate::models::media::Media;
use crate::repositories::media_repository::MediaRepository;
use warp::reject::Reject;

pub struct MediaService;

#[derive(Debug)]
struct MediaError;
impl Reject for MediaError {}

impl MediaService {
    pub async fn upload_media(new_media: Media) -> Result<Media, warp::Rejection> {
        MediaRepository::create(new_media).map_err(|_| warp::reject::custom(MediaError))
    }

    pub async fn get_media(id: i32) -> Result<Media, warp::Rejection> {
        MediaRepository::find_by_id(id).map_err(|_| warp::reject::custom(MediaError))
    }

    pub async fn delete_media(id: i32) -> Result<(), warp::Rejection> {
        MediaRepository::delete(id).map_err(|_| warp::reject::custom(MediaError))
    }

    pub async fn list_media() -> Result<Vec<Media>, warp::Rejection> {
        MediaRepository::find_all().map_err(|_| warp::reject::custom(MediaError))
    }
}
