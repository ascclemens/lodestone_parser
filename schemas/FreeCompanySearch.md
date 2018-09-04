# FreeCompanySearch

|Key|Value|Description|
|---|---|---|
|`pagination`|`Pagination`|Pagination info. See Pagination section below.|
|`results`|`Array` of `FreeCompanySearchItem`|See FreeCompanySearchItem section below.|

## FreeCompanySearchItem

|Key|Value|Description|
|---|---|---|
|`id`|`u64`|The Free Company's Lodestone ID.|
|`name`|`String`|The Free Company's name.|
|`world`|`String`|The world the Free Company is on.|
|`crest`|`Array` of `String` (URLs)|An array of URLs pointing to images that can be overlayed to create the Free Company crest.|
|`grand_company`|`GrandCompany`|The Grand Company that the Free Company is affiliated with.|
|`active_members`|`u16`|The number of members in the Free Company.|
|`estate_built`|`bool`|If the Free Company has an estate.|
|`formed`|`DateTime` (UTC)|The date and time at which the Free Company was created.|
|`active`|`Active`|When often the Free Company is active. See Active section below.|
|`recruitment`|`RecruitmentStatus`|The Free Company's recruitment status. See the RecruitmentStatus section below.|

## Pagination

|Key|Value|Description|
|---|---|---|
|`current_page`|`u64`|The current page represented in the results.|
|`total_pages`|`u64`|The total amount of pages available for the search query.|
|`total_results`|`u64`|The total amount of results across all pages for the search query.|

## Active

An enumerated type represented as a string.

|Active|String|
|---|---|
|Not Specified|`"NotSpecified"`|
|Always|`"Always"`|
|Weekends|`"Weekends"`|
|Weekdays|`"Weekdays"`|

## RecruitmentStats

An enumerated type represented as a string.

|RecruitmentStats|String|
|---|---|
|Open|`"Open"`|
|Closed|`"Closed"`|
