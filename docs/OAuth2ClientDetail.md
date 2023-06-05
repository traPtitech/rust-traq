# OAuth2ClientDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | クライアントUUID | 
**developer_id** | [**uuid::Uuid**](uuid::Uuid.md) | クライアント開発者UUID | 
**description** | **String** | 説明 | 
**name** | **String** | クライアント名 | 
**scopes** | [**Vec<crate::models::OAuth2Scope>**](OAuth2Scope.md) | 要求スコープの配列 | 
**callback_url** | **String** | コールバックURL | 
**secret** | **String** | クライアントシークレット | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


