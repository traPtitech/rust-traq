# FileInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ファイルUUID | 
**name** | **String** | ファイル名 | 
**mime** | **String** | MIMEタイプ | 
**size** | **i64** | ファイルサイズ | 
**md5** | **String** | MD5ハッシュ | 
**is_animated_image** | **bool** | アニメーション画像かどうか | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | アップロード日時 | 
**thumbnails** | [**Vec<models::ThumbnailInfo>**](ThumbnailInfo.md) |  | 
**thumbnail** | Option<[**models::FileInfoThumbnail**](FileInfoThumbnail.md)> |  | 
**channel_id** | Option<**uuid::Uuid**> | 属しているチャンネルUUID | 
**uploader_id** | Option<**uuid::Uuid**> | アップロード者UUID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


