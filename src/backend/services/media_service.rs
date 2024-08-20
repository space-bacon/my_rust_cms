use crate::models::media::Media;
use crate::repositories::media_repository::MediaRepository;

pub struct MediaService;

impl MediaService {
    pub fn upload_media(new_media: Media) -> Result<Media, &'static str> {
        MediaRepository::create(new_media)
    }

    pub fn get_media(id: i32) -> Result<Media, &'static str> {
        MediaRepository::find_by_id(id)
    }

    pub fn delete_media(id: i32) -> Result<(), &'static str> {
        MediaRepository::delete(id)
    }

    pub fn list_media() -> Result<Vec<Media>, &'static str> {
        MediaRepository::find_all()
    }
}
