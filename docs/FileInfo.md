# FileInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | ファイルUUID | 
**name** | **String** | ファイル名 | 
**mime** | **String** | MIMEタイプ | 
**size** | **i64** | ファイルサイズ | 
**md5** | **String** | MD5ハッシュ | 
**is_animated_image** | **bool** | アニメーション画像かどうか | 
**created_at** | **String** | アップロード日時 | 
**thumbnails** | [**Vec<crate::models::ThumbnailInfo>**](ThumbnailInfo.md) |  | 
**thumbnail** | Option<[**crate::models::FileInfoThumbnail**](FileInfo_thumbnail.md)> |  | 
**channel_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | 属しているチャンネルUUID | 
**uploader_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | アップロード者UUID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


