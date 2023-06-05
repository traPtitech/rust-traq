# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | チャンネルUUID | 
**parent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | 親チャンネルUUID | 
**archived** | **bool** | チャンネルがアーカイブされているかどうか | 
**force** | **bool** | 強制通知チャンネルかどうか | 
**topic** | **String** | チャンネルトピック | 
**name** | **String** | チャンネル名 | 
**children** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | 子チャンネルのUUID配列 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


