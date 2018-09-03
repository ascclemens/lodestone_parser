# CharacterSearch

|Key|Value|Description|
|---|---|---|
|`pagination`|`Pagination`|Pagination info. See Pagination section below.|
|`results`|`Array` of `CharacterSearchItem`|See CharacterSearchItem section below.|

## CharacterSearchItem

|Key|Value|Description|
|---|---|---|
|`id`|`u64`|The character's Lodestone ID.|
|`name`|`String`|The character's name.|
|`world`|`String`|The world the character is on.|
|`grand_company`|`GrandCompanyInfo?`|The character's Grand Company affiliation and rank. See GrandCompanyInfo section on the Character schema page.|
|`free_company_id`|`u64?`|The ID of the character's Free Company, if any.|
|`face`|`String` (URL)|A URL to a picture of the character's face.|

## Pagination

|Key|Value|Description|
|---|---|---|
|`current_page`|`u64`|The current page represented in the results.|
|`total_pages`|`u64`|The total amount of pages available for the search query.|
|`total_results`|`u64`|The total amount of results across all pages for the search query.|
