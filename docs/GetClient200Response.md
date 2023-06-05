# GetClient200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | クライアントUUID | 
**name** | **String** | クライアント名 | 
**description** | **String** | 説明 | 
**developer_id** | [**uuid::Uuid**](uuid::Uuid.md) | クライアント開発者UUID | 
**scopes** | [**Vec<crate::models::OAuth2Scope>**](OAuth2Scope.md) | 要求スコープの配列 | 
**callback_url** | **String** | コールバックURL | 
**secret** | **String** | クライアントシークレット | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


