# FreeCompany

|Key|Value|Description|
|---|---|---|
|`name`|`String`|The name of the Free Company.|
|`world`|`String`|The world the Free Company is on.|
|`slogan`|`String`|The Free Company's slogan.|
|`crest`|`Array` of `String`|The image URLs that are layered to created the Free Company crest.|
|`grand_company`|`String`|The Grand Company that the Free Company is affiliated with. Will be one of `"Flames"`, `"TwinAdders"`, or `"Maelstrom"`.|
|`active_members`|`u16`|The amount of active members.
|`rank`|`u8`|The Free Company's rank ([1,8]).|
|`pvp_rankings`|`PvpRankings`|The Free Company's PvP rankings.|
|`formed`|`DateTime` (UTC, RFC3339 formatted)|The date and time at which the Free Company was created.|
|`estate`|`Estate?`|The Free Company's estate.|
|`reputation`|`Map` of `String` to `u8`|The reputation the Free Company has with each Grand Company. The keys are all possible values of the `grand_company` field.|

## PvpRankings

|Key|Value|Description|
|---|---|---|
|`weekly`|`u64?`|The weekly rank or `null` if unranked.|
|`monthly`|`u64?`|The monthly rank or `null` if unranked.|

## Estate

|Key|Value|Description|
|---|---|---|
|`name`|`String`|The name of the estate.|
|`address`|`String`|The estate's address.|
|`greeting`|`String`|The greeting set on the estate.|
